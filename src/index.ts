import { NativeModules } from 'react-native';

const { ElectrumClient } = NativeModules;

if (!ElectrumClient) {
  throw new Error(
    'ElectrumClient native module is not available. Make sure you have linked the module correctly.'
  );
}

export interface Peer {
  host: string;
  ssl?: number;
  tcp?: number;
  protocol?: string;
}

export interface StartConfig {
  network: string;
  customPeers?: Peer[];
}

export interface ElectrumResponse {
  error: boolean;
  data?: string;
  method?: string;
}

export interface ElectrumClientNative {
  start(config: StartConfig): Promise<ElectrumResponse>;
  stop(network: string): Promise<ElectrumResponse>;
  pingServer(network: string): Promise<ElectrumResponse>;
  getHeader(network: string, height: number): Promise<ElectrumResponse>;
  getBalance(network: string, scriptHashes: string[]): Promise<ElectrumResponse>;
  subscribeHeader(network: string): Promise<ElectrumResponse>;
  getAddressScriptHashesHistory(network: string, scriptHashes: string[]): Promise<ElectrumResponse>;
  getTransactions(network: string, txHashes: string[]): Promise<ElectrumResponse>;
  getTransactionMerkle(network: string, txHash: string, height: number): Promise<ElectrumResponse>;
  broadcastTransaction(network: string, rawTx: string): Promise<ElectrumResponse>;
}

const nativeModule: ElectrumClientNative = ElectrumClient;

/**
 * Start a connection to an Electrum server
 * @param config - Configuration object containing network and optional custom peers
 * @returns Promise resolving to ElectrumResponse
 */
export async function start(config: StartConfig): Promise<ElectrumResponse> {
  return nativeModule.start(config);
}

/**
 * Stop the connection to an Electrum server
 * @param network - Network name (e.g., 'bitcoin', 'bitcoinTestnet')
 * @returns Promise resolving to ElectrumResponse
 */
export async function stop(network: string): Promise<ElectrumResponse> {
  return nativeModule.stop(network);
}

/**
 * Ping the connected Electrum server
 * @param network - Network name
 * @returns Promise resolving to ElectrumResponse
 */
export async function pingServer(network: string): Promise<ElectrumResponse> {
  return nativeModule.pingServer(network);
}

/**
 * Get a block header at a specific height
 * @param network - Network name
 * @param height - Block height
 * @returns Promise resolving to ElectrumResponse containing the block header
 */
export async function getHeader(
  network: string,
  height: number
): Promise<ElectrumResponse> {
  return nativeModule.getHeader(network, height);
}

/**
 * Get balance for multiple script hashes
 * @param network - Network name
 * @param scriptHashes - Array of script hashes to check
 * @returns Promise resolving to ElectrumResponse containing balance information
 */
export async function getBalance(
  network: string,
  scriptHashes: string[]
): Promise<ElectrumResponse> {
  return nativeModule.getBalance(network, scriptHashes);
}

/**
 * Subscribe to blockchain headers for updates
 * @param network - Network name
 * @returns Promise resolving to ElectrumResponse containing current header
 */
export async function subscribeHeader(
  network: string
): Promise<ElectrumResponse> {
  return nativeModule.subscribeHeader(network);
}

/**
 * Get transaction history for multiple script hashes
 * @param network - Network name
 * @param scriptHashes - Array of script hashes
 * @returns Promise resolving to ElectrumResponse containing transaction history
 */
export async function getAddressScriptHashesHistory(
  network: string,
  scriptHashes: string[]
): Promise<ElectrumResponse> {
  return nativeModule.getAddressScriptHashesHistory(network, scriptHashes);
}

/**
 * Get full transaction data for multiple transactions
 * @param network - Network name
 * @param txHashes - Array of transaction hashes
 * @returns Promise resolving to ElectrumResponse containing transaction data
 */
export async function getTransactions(
  network: string,
  txHashes: string[]
): Promise<ElectrumResponse> {
  return nativeModule.getTransactions(network, txHashes);
}

/**
 * Get merkle proof for a transaction
 * @param network - Network name
 * @param txHash - Transaction hash
 * @param height - Block height
 * @returns Promise resolving to ElectrumResponse containing merkle proof
 */
export async function getTransactionMerkle(
  network: string,
  txHash: string,
  height: number
): Promise<ElectrumResponse> {
  return nativeModule.getTransactionMerkle(network, txHash, height);
}

/**
 * Broadcast a raw transaction to the network
 * @param network - Network name
 * @param rawTx - Raw transaction hex
 * @returns Promise resolving to ElectrumResponse containing transaction ID
 */
export async function broadcastTransaction(
  network: string,
  rawTx: string
): Promise<ElectrumResponse> {
  return nativeModule.broadcastTransaction(network, rawTx);
}

export default {
  start,
  stop,
  pingServer,
  getHeader,
  getBalance,
  subscribeHeader,
  getAddressScriptHashesHistory,
  getTransactions,
  getTransactionMerkle,
  broadcastTransaction,
};
