use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use native_tls::TlsConnector;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;

uniffi::include_scaffolding!("electrum_client");

// Error types
#[derive(Debug, Error)]
pub enum ElectrumError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Invalid network: {0}")]
    InvalidNetwork(String),
    #[error("Invalid response from server")]
    InvalidResponse,
    #[error("Request timeout")]
    Timeout,
    #[error("Already connected to network")]
    AlreadyConnected,
    #[error("Not connected to network")]
    NotConnected,
    #[error("Other error: {0}")]
    Other(String),
}

// Data structures matching the UDL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartConfig {
    pub network: String,
    pub custom_peers: Option<Vec<Peer>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub host: String,
    pub ssl: Option<u16>,
    pub tcp: Option<u16>,
    pub protocol: Option<String>,
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

    fn error(method: String) -> Self {
        Self {
            error: true,
            data: None,
            method: Some(method),
        }
    }
}

// Connection manager
struct ElectrumConnection {
    stream: Box<dyn ElectrumStream>,
    request_id: u64,
}

trait ElectrumStream: Send {
    fn send_request(&mut self, method: &str, params: Value) -> Result<Value, ElectrumError>;
}

struct TlsElectrumStream {
    stream: BufReader<native_tls::TlsStream<TcpStream>>,
}

impl ElectrumStream for TlsElectrumStream {
    fn send_request(&mut self, method: &str, params: Value) -> Result<Value, ElectrumError> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let request_str = serde_json::to_string(&request)
            .map_err(|e| ElectrumError::Other(format!("Serialization error: {}", e)))?;

        // Send request
        self.stream
            .get_mut()
            .write_all(format!("{}\n", request_str).as_bytes())
            .map_err(|e| ElectrumError::ConnectionFailed(format!("Write error: {}", e)))?;

        self.stream
            .get_mut()
            .flush()
            .map_err(|e| ElectrumError::ConnectionFailed(format!("Flush error: {}", e)))?;

        // Read response
        let mut response_str = String::new();
        self.stream
            .read_line(&mut response_str)
            .map_err(|e| ElectrumError::ConnectionFailed(format!("Read error: {}", e)))?;

        let response: Value = serde_json::from_str(&response_str)
            .map_err(|_e| ElectrumError::InvalidResponse)?;

        if let Some(error) = response.get("error") {
            if !error.is_null() {
                return Err(ElectrumError::Other(format!(
                    "Server error: {}",
                    error
                )));
            }
        }

        response
            .get("result")
            .cloned()
            .ok_or(ElectrumError::InvalidResponse)
    }
}

// Global connection manager
lazy_static::lazy_static! {
    static ref CONNECTIONS: Arc<Mutex<HashMap<String, ElectrumConnection>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// Default peers for different networks
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
            Peer {
                host: "fortress.qtornado.com".to_string(),
                ssl: Some(443),
                tcp: Some(50001),
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

// Public API functions
pub fn start(config: StartConfig) -> Result<ElectrumResponse, ElectrumError> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| ElectrumError::Other(format!("Lock error: {}", e)))?;

    if connections.contains_key(&config.network) {
        return Err(ElectrumError::AlreadyConnected);
    }

    let peers = config.custom_peers.unwrap_or_else(|| get_default_peers(&config.network));

    if peers.is_empty() {
        return Err(ElectrumError::InvalidNetwork(
            "No peers available for network".to_string(),
        ));
    }

    // Try to connect to each peer
    for peer in peers {
        let protocol = peer.protocol.as_deref().unwrap_or("ssl");

        if protocol == "ssl" {
            if let Some(port) = peer.ssl {
                match connect_tls(&peer.host, port) {
                    Ok(mut stream) => {
                        // Verify connection with server.version call
                        match stream.send_request("server.version", json!(["react-native-electrum-client-rs", "1.4"])) {
                            Ok(version) => {
                                let connection = ElectrumConnection {
                                    stream: Box::new(stream),
                                    request_id: 1,
                                };
                                connections.insert(config.network.clone(), connection);

                                return Ok(ElectrumResponse::success(
                                    serde_json::to_string(&version).unwrap_or_default()
                                ));
                            }
                            Err(e) => {
                                log::warn!("Server version check failed for {}: {}", peer.host, e);
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to connect to {}:{}: {}", peer.host, port, e);
                        continue;
                    }
                }
            }
        }
    }

    Err(ElectrumError::ConnectionFailed(
        "Could not connect to any peer".to_string(),
    ))
}

pub fn stop(network: String) -> Result<ElectrumResponse, ElectrumError> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| ElectrumError::Other(format!("Lock error: {}", e)))?;

    connections.remove(&network);

    Ok(ElectrumResponse::success("Disconnected".to_string()))
}

pub fn ping_server(network: String) -> Result<ElectrumResponse, ElectrumError> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| ElectrumError::Other(format!("Lock error: {}", e)))?;

    let connection = connections
        .get_mut(&network)
        .ok_or(ElectrumError::NotConnected)?;

    let result = connection.stream.send_request("server.ping", json!([]))?;

    Ok(ElectrumResponse::success(
        serde_json::to_string(&result).unwrap_or_default(),
    ))
}

pub fn get_header(network: String, height: u32) -> Result<ElectrumResponse, ElectrumError> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| ElectrumError::Other(format!("Lock error: {}", e)))?;

    let connection = connections
        .get_mut(&network)
        .ok_or(ElectrumError::NotConnected)?;

    let result = connection
        .stream
        .send_request("blockchain.block.header", json!([height]))?;

    Ok(ElectrumResponse::success(
        serde_json::to_string(&result).unwrap_or_default(),
    ))
}

pub fn get_balance(network: String, script_hashes: Vec<String>) -> Result<ElectrumResponse, ElectrumError> {
    let mut connections = CONNECTIONS
        .lock()
        .map_err(|e| ElectrumError::Other(format!("Lock error: {}", e)))?;

    let connection = connections
        .get_mut(&network)
        .ok_or(ElectrumError::NotConnected)?;

    let mut balances = json!({});

    for script_hash in script_hashes {
        let result = connection
            .stream
            .send_request("blockchain.scripthash.get_balance", json!([script_hash]))?;

        balances[&script_hash] = result;
    }

    Ok(ElectrumResponse::success(
        serde_json::to_string(&balances).unwrap_or_default(),
    ))
}

// Helper function to connect via TLS
fn connect_tls(host: &str, port: u16) -> Result<TlsElectrumStream, ElectrumError> {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(false)
        .build()
        .map_err(|e| ElectrumError::ConnectionFailed(format!("TLS builder error: {}", e)))?;

    let tcp_stream = TcpStream::connect((host, port))
        .map_err(|e| ElectrumError::ConnectionFailed(format!("TCP connection error: {}", e)))?;

    tcp_stream
        .set_read_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| ElectrumError::ConnectionFailed(format!("Set timeout error: {}", e)))?;

    tcp_stream
        .set_write_timeout(Some(Duration::from_secs(30)))
        .map_err(|e| ElectrumError::ConnectionFailed(format!("Set timeout error: {}", e)))?;

    let tls_stream = connector
        .connect(host, tcp_stream)
        .map_err(|e| ElectrumError::ConnectionFailed(format!("TLS connection error: {}", e)))?;

    let stream = BufReader::new(tls_stream);

    Ok(TlsElectrumStream { stream })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() {
        let config = StartConfig {
            network: "bitcoin".to_string(),
            custom_peers: Some(vec![Peer {
                host: "electrum.blockstream.info".to_string(),
                ssl: Some(50002),
                tcp: None,
                protocol: Some("ssl".to_string()),
            }]),
        };

        let result = start(config);
        assert!(result.is_ok());

        if let Ok(response) = result {
            assert!(!response.error);
        }
    }
}
