package com.electrumclientrs

import com.facebook.react.bridge.*
import com.facebook.react.module.annotations.ReactModule

@ReactModule(name = ElectrumClientModule.NAME)
class ElectrumClientModule(reactContext: ReactApplicationContext) :
    ReactContextBaseJavaModule(reactContext) {

    override fun getName(): String = NAME

    @ReactMethod
    fun start(config: ReadableMap, promise: Promise) {
        try {
            val network = config.getString("network") ?: "bitcoin"
            val customPeers = config.getArray("customPeers")?.let { peersArray ->
                val peers = mutableListOf<Peer>()
                for (i in 0 until peersArray.size()) {
                    val peerMap = peersArray.getMap(i)
                    if (peerMap != null) {
                        peers.add(
                            Peer(
                                host = peerMap.getString("host") ?: "",
                                ssl = if (peerMap.hasKey("ssl")) peerMap.getInt("ssl").toUShort() else null,
                                tcp = if (peerMap.hasKey("tcp")) peerMap.getInt("tcp").toUShort() else null,
                                protocol = peerMap.getString("protocol")
                            )
                        )
                    }
                }
                peers
            }

            val startConfig = StartConfig(
                network = network,
                customPeers = customPeers
            )

            val result = uniffi.electrum_client_rs.start(startConfig)
            val response = Arguments.createMap().apply {
                putBoolean("error", result.error)
                putString("data", result.data)
                putString("method", result.method)
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun stop(network: String, promise: Promise) {
        try {
            val result = uniffi.electrum_client_rs.stop(network)
            val response = Arguments.createMap().apply {
                putBoolean("error", result.error)
                putString("data", result.data)
                putString("method", result.method)
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun pingServer(network: String, promise: Promise) {
        try {
            val result = uniffi.electrum_client_rs.pingServer(network)
            val response = Arguments.createMap().apply {
                putBoolean("error", result.error)
                putString("data", result.data)
                putString("method", result.method)
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun getHeader(network: String, height: Int, promise: Promise) {
        try {
            val result = uniffi.electrum_client_rs.getHeader(network, height.toUInt())
            val response = Arguments.createMap().apply {
                putBoolean("error", result.error)
                putString("data", result.data)
                putString("method", result.method)
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    @ReactMethod
    fun getBalance(network: String, scriptHashes: ReadableArray, promise: Promise) {
        try {
            val hashes = mutableListOf<String>()
            for (i in 0 until scriptHashes.size()) {
                scriptHashes.getString(i)?.let { hashes.add(it) }
            }

            val result = uniffi.electrum_client_rs.getBalance(network, hashes)
            val response = Arguments.createMap().apply {
                putBoolean("error", result.error)
                putString("data", result.data)
                putString("method", result.method)
            }
            promise.resolve(response)
        } catch (e: Exception) {
            promise.reject("ELECTRUM_ERROR", e.message, e)
        }
    }

    companion object {
        const val NAME = "ElectrumClient"

        init {
            System.loadLibrary("electrum_client_rs")
        }
    }
}
