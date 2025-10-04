use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[cfg(target_os = "android")]
use jni::JNIEnv;
#[cfg(target_os = "android")]
use jni::objects::{JClass, JString, JObject};
#[cfg(target_os = "android")]
use jni::sys::jstring;

// Data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub host: String,
    pub ssl: Option<u16>,
    pub tcp: Option<u16>,
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartConfig {
    pub network: String,
    pub custom_peers: Option<Vec<Peer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElectrumResponse {
    pub error: bool,
    pub data: Option<String>,
    pub method: Option<String>,
}

impl ElectrumResponse {
    fn success(data: String) -> Self {
        Self {
            error: false,
            data: Some(data),
            method: None,
        }
    }

    fn error_response(method: String) -> Self {
        Self {
            error: true,
            data: None,
            method: Some(method),
        }
    }
}

// Connection manager
struct ElectrumConnection {
    stream: BufReader<native_tls::TlsStream<TcpStream>>,
}

impl ElectrumConnection {
    fn send_request(&mut self, method: &str, params: Value) -> Result<Value, String> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let request_str = serde_json::to_string(&request)
            .map_err(|e| format!("Serialization error: {}", e))?;

        // Send request
        self.stream
            .get_mut()
            .write_all(format!("{}\n", request_str).as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;

        self.stream
            .get_mut()
            .flush()
            .map_err(|e| format!("Flush error: {}", e))?;

        // Read response
        let mut response_str = String::new();
        self.stream
            .read_line(&mut response_str)
            .map_err(|e| format!("Read error: {}", e))?;

        let response: Value = serde_json::from_str(&response_str)
            .map_err(|_| "Invalid response".to_string())?;

        if let Some(error) = response.get("error") {
            if !error.is_null() {
                return Err(format!("Server error: {}", error));
            }
        }

        response
            .get("result")
            .cloned()
            .ok_or_else(|| "Invalid response".to_string())
    }
}

// Global connection manager
lazy_static::lazy_static! {
    static ref CONNECTIONS: Arc<Mutex<HashMap<String, ElectrumConnection>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// Default peers
fn get_default_peers(network: &str) -> Vec<Peer> {
    match network {
        "bitcoin" => vec![
            Peer {
                host: "electrum.blockstream.info".to_string(),
                ssl: Some(50002),
                tcp: Some(50001),
                protocol: Some("ssl".to_string()),
            },
            Peer {
                host: "blockstream.info".to_string(),
                ssl: Some(700),
                tcp: Some(110),
                protocol: Some("ssl".to_string()),
            },
        ],
        "bitcoinTestnet" => vec![Peer {
            host: "testnet.aranguren.org".to_string(),
            ssl: Some(51002),
            tcp: Some(51001),
            protocol: Some("ssl".to_string()),
        }],
        _ => vec![],
    }
}

// Core functions
pub fn start_impl(config: StartConfig) -> Result<ElectrumResponse, String> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    if connections.contains_key(&config.network) {
        return Err("Already connected".to_string());
    }

    let peers = config.custom_peers.unwrap_or_else(|| get_default_peers(&config.network));

    if peers.is_empty() {
        return Err("No peers available".to_string());
    }

    for peer in peers {
        let protocol = peer.protocol.as_deref().unwrap_or("ssl");

        if protocol == "ssl" {
            if let Some(port) = peer.ssl {
                match connect_tls(&peer.host, port) {
                    Ok(mut stream) => {
                        match stream.send_request("server.version", json!(["react-native-electrum-client-rs", "1.4"])) {
                            Ok(version) => {
                                connections.insert(config.network.clone(), stream);
                                return Ok(ElectrumResponse::success(
                                    serde_json::to_string(&version).unwrap_or_default()
                                ));
                            }
                            Err(_) => continue,
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }

    Err("Could not connect to any peer".to_string())
}

pub fn stop_impl(network: String) -> Result<ElectrumResponse, String> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    connections.remove(&network);
    Ok(ElectrumResponse::success("Disconnected".to_string()))
}

pub fn ping_server_impl(network: String) -> Result<ElectrumResponse, String> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let connection = connections
        .get_mut(&network)
        .ok_or("Not connected".to_string())?;

    let result = connection.send_request("server.ping", json!([]))?;

    Ok(ElectrumResponse::success(
        serde_json::to_string(&result).unwrap_or_default(),
    ))
}

pub fn get_header_impl(network: String, height: u32) -> Result<ElectrumResponse, String> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let connection = connections
        .get_mut(&network)
        .ok_or("Not connected".to_string())?;

    let result = connection
        .send_request("blockchain.block.header", json!([height]))?;

    Ok(ElectrumResponse::success(
        serde_json::to_string(&result).unwrap_or_default(),
    ))
}

pub fn get_balance_impl(network: String, script_hashes: Vec<String>) -> Result<ElectrumResponse, String> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| format!("Lock error: {}", e))?;

    let connection = connections
        .get_mut(&network)
        .ok_or("Not connected".to_string())?;

    let mut balances = json!({});

    for script_hash in script_hashes {
        let result = connection
            .send_request("blockchain.scripthash.get_balance", json!([script_hash]))?;

        balances[&script_hash] = result;
    }

    Ok(ElectrumResponse::success(
        serde_json::to_string(&balances).unwrap_or_default(),
    ))
}

fn connect_tls(host: &str, port: u16) -> Result<ElectrumConnection, String> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| format!("TLS builder error: {}", e))?;

    let tcp_stream = TcpStream::connect((host, port))
        .map_err(|e| format!("TCP connection error: {}", e))?;

    tcp_stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| format!("Set timeout error: {}", e))?;

    tcp_stream
        .set_write_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| format!("Set timeout error: {}", e))?;

    let tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| format!("TLS connection error: {}", e))?;

    let stream = BufReader::new(tls_stream);

    Ok(ElectrumConnection { stream })
}

// Android JNI bindings
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_electrumclientrs_ElectrumClientModule_nativeStart(
    env: JNIEnv,
    _: JClass,
    config_json: JString,
) -> jstring {
    let config_str: String = env.get_string(config_json).unwrap().into();

    let result = match serde_json::from_str::<StartConfig>(&config_str) {
        Ok(config) => match start_impl(config) {
            Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
        },
        Err(_) => serde_json::to_string(&ElectrumResponse::error_response("Invalid config".to_string())).unwrap_or_else(|_| "{}".to_string()),
    };

    env.new_string(result).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_electrumclientrs_ElectrumClientModule_nativeStop(
    env: JNIEnv,
    _: JClass,
    network: JString,
) -> jstring {
    let network_str: String = env.get_string(network).unwrap().into();

    let result = match stop_impl(network_str) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    env.new_string(result).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_electrumclientrs_ElectrumClientModule_nativePingServer(
    env: JNIEnv,
    _: JClass,
    network: JString,
) -> jstring {
    let network_str: String = env.get_string(network).unwrap().into();

    let result = match ping_server_impl(network_str) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    env.new_string(result).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_electrumclientrs_ElectrumClientModule_nativeGetHeader(
    env: JNIEnv,
    _: JClass,
    network: JString,
    height: i32,
) -> jstring {
    let network_str: String = env.get_string(network).unwrap().into();

    let result = match get_header_impl(network_str, height as u32) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    env.new_string(result).unwrap().into_raw()
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_electrumclientrs_ElectrumClientModule_nativeGetBalance(
    env: JNIEnv,
    _: JClass,
    network: JString,
    script_hashes_json: JString,
) -> jstring {
    let network_str: String = env.get_string(network).unwrap().into();
    let hashes_str: String = env.get_string(script_hashes_json).unwrap().into();

    let result = match serde_json::from_str::<Vec<String>>(&hashes_str) {
        Ok(hashes) => match get_balance_impl(network_str, hashes) {
            Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
        },
        Err(_) => serde_json::to_string(&ElectrumResponse::error_response("Invalid script hashes".to_string())).unwrap_or_else(|_| "{}".to_string()),
    };

    env.new_string(result).unwrap().into_raw()
}

// iOS C FFI bindings
#[no_mangle]
pub extern "C" fn electrum_start(config_json: *const c_char) -> *mut c_char {
    let config_str = unsafe { CStr::from_ptr(config_json).to_str().unwrap() };

    let result = match serde_json::from_str::<StartConfig>(config_str) {
        Ok(config) => match start_impl(config) {
            Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
        },
        Err(_) => serde_json::to_string(&ElectrumResponse::error_response("Invalid config".to_string())).unwrap_or_else(|_| "{}".to_string()),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn electrum_stop(network: *const c_char) -> *mut c_char {
    let network_str = unsafe { CStr::from_ptr(network).to_str().unwrap() }.to_string();

    let result = match stop_impl(network_str) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn electrum_ping_server(network: *const c_char) -> *mut c_char {
    let network_str = unsafe { CStr::from_ptr(network).to_str().unwrap() }.to_string();

    let result = match ping_server_impl(network_str) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn electrum_get_header(network: *const c_char, height: u32) -> *mut c_char {
    let network_str = unsafe { CStr::from_ptr(network).to_str().unwrap() }.to_string();

    let result = match get_header_impl(network_str, height) {
        Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
        Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn electrum_get_balance(network: *const c_char, script_hashes_json: *const c_char) -> *mut c_char {
    let network_str = unsafe { CStr::from_ptr(network).to_str().unwrap() }.to_string();
    let hashes_str = unsafe { CStr::from_ptr(script_hashes_json).to_str().unwrap() };

    let result = match serde_json::from_str::<Vec<String>>(hashes_str) {
        Ok(hashes) => match get_balance_impl(network_str, hashes) {
            Ok(response) => serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()),
            Err(e) => serde_json::to_string(&ElectrumResponse::error_response(e)).unwrap_or_else(|_| "{}".to_string()),
        },
        Err(_) => serde_json::to_string(&ElectrumResponse::error_response("Invalid script hashes".to_string())).unwrap_or_else(|_| "{}".to_string()),
    };

    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn electrum_free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        let _ = CString::from_raw(s);
    }
}
