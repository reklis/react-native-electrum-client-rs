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

export default {
  start,
  stop,
  pingServer,
  getHeader,
  getBalance,
};
