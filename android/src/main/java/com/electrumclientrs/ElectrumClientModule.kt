package com.electrumclientrs

import com.facebook.react.bridge.*
import com.facebook.react.module.annotations.ReactModule
import org.json.JSONArray
import org.json.JSONObject

@ReactModule(name = ElectrumClientModule.NAME)
class ElectrumClientModule(reactContext: ReactApplicationContext) :
    ReactContextBaseJavaModule(reactContext) {

    override fun getName(): String = NAME

    @ReactMethod
    fun start(config: ReadableMap, promise: Promise) {
        try {
            val configJson = JSONObject().apply {
                put("network", config.getString("network") ?: "bitcoin")
                if (config.hasKey("customPeers")) {
                    val peersArray = config.getArray("customPeers")
                    val peersJsonArray = JSONArray()
                    if (peersArray != null) {
                        for (i in 0 until peersArray.size()) {
                            val peer = peersArray.getMap(i)
                            if (peer != null) {
                                val peerJson = JSONObject().apply {
                                    put("host", peer.getString("host") ?: "")
                                    if (peer.hasKey("ssl")) put("ssl", peer.getInt("ssl"))
                                    if (peer.hasKey("tcp")) put("tcp", peer.getInt("tcp"))
                                    if (peer.hasKey("protocol")) put("protocol", peer.getString("protocol"))
                                }
                                peersJsonArray.put(peerJson)
                            }
                        }
                    }
                    put("custom_peers", peersJsonArray)
                }
            }

            val resultJson = nativeStart(configJson.toString())
            val result = JSONObject(resultJson)

            val response = Arguments.createMap().apply {
                putBoolean("error", result.getBoolean("error"))
                if (result.has("data") && !result.isNull("data")) {
                    putString("data", result.getString("data"))
                }
                if (result.has("method") && !result.isNull("method")) {
                    putString("method", result.getString("method"))
                }
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun stop(network: String, promise: Promise) {
        try {
            val resultJson = nativeStop(network)
            val result = JSONObject(resultJson)

            val response = Arguments.createMap().apply {
                putBoolean("error", result.getBoolean("error"))
                if (result.has("data") && !result.isNull("data")) {
                    putString("data", result.getString("data"))
                }
                if (result.has("method") && !result.isNull("method")) {
                    putString("method", result.getString("method"))
                }
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun pingServer(network: String, promise: Promise) {
        try {
            val resultJson = nativePingServer(network)
            val result = JSONObject(resultJson)

            val response = Arguments.createMap().apply {
                putBoolean("error", result.getBoolean("error"))
                if (result.has("data") && !result.isNull("data")) {
                    putString("data", result.getString("data"))
                }
                if (result.has("method") && !result.isNull("method")) {
                    putString("method", result.getString("method"))
                }
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun getHeader(network: String, height: Int, promise: Promise) {
        try {
            val resultJson = nativeGetHeader(network, height)
            val result = JSONObject(resultJson)

            val response = Arguments.createMap().apply {
                putBoolean("error", result.getBoolean("error"))
                if (result.has("data") && !result.isNull("data")) {
                    putString("data", result.getString("data"))
                }
                if (result.has("method") && !result.isNull("method")) {
                    putString("method", result.getString("method"))
                }
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun getBalance(network: String, scriptHashes: ReadableArray, promise: Promise) {
        try {
            val hashes = JSONArray()
            for (i in 0 until scriptHashes.size()) {
                scriptHashes.getString(i)?.let { hashes.put(it) }
            }

            val resultJson = nativeGetBalance(network, hashes.toString())
            val result = JSONObject(resultJson)

            val response = Arguments.createMap().apply {
                putBoolean("error", result.getBoolean("error"))
                if (result.has("data") && !result.isNull("data")) {
                    putString("data", result.getString("data"))
                }
                if (result.has("method") && !result.isNull("method")) {
                    putString("method", result.getString("method"))
                }
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    // Native JNI methods
    private external fun nativeStart(configJson: String): String
    private external fun nativeStop(network: String): String
    private external fun nativePingServer(network: String): String
    private external fun nativeGetHeader(network: String, height: Int): String
    private external fun nativeGetBalance(network: String, scriptHashesJson: String): String

    companion object {
        const val NAME = "ElectrumClient"

        init {
            System.loadLibrary("electrum_client_rs")
        }
    }
}
