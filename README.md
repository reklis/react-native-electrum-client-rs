# react-native-electrum-client-rs

High-performance Electrum client for React Native, powered by Rust.

## Features

- 🚀 **Native Performance**: Core functionality written in Rust for maximum performance
- 🔒 **Type-Safe**: Full TypeScript support with comprehensive type definitions
- 📱 **Cross-Platform**: Works on both iOS and Android
- ⚡ **Async/Await**: Modern Promise-based API
- 🔐 **Secure**: TLS/SSL support for secure connections to Electrum servers

## Installation

```bash
npm install react-native-electrum-client-rs
# or
yarn add react-native-electrum-client-rs
```

### iOS

```bash
cd ios && pod install
```

### Android

No additional steps required.

## Usage

```typescript
import ElectrumClient from 'react-native-electrum-client-rs';

// Start connection
const response = await ElectrumClient.start({
  network: 'bitcoin',
  customPeers: [
    {
      host: 'electrum.blockstream.info',
      ssl: 50002,
      protocol: 'ssl'
    }
  ]
});

if (!response.error) {
  console.log('Connected!', response.data);
}

// Ping server
const pingResponse = await ElectrumClient.pingServer('bitcoin');

// Get block header
const headerResponse = await ElectrumClient.getHeader('bitcoin', 1);

// Get balance
const balanceResponse = await ElectrumClient.getBalance('bitcoin', [
  'your_script_hash_here'
]);

// Stop connection
await ElectrumClient.stop('bitcoin');
```

## API

### `start(config: StartConfig): Promise<ElectrumResponse>`

Start a connection to an Electrum server.

**Parameters:**
- `config.network` - Network name ('bitcoin', 'bitcoinTestnet', etc.)
- `config.customPeers` - Optional array of custom peers to connect to

**Returns:** Promise resolving to `ElectrumResponse`

### `stop(network: string): Promise<ElectrumResponse>`

Stop the connection to an Electrum server.

### `pingServer(network: string): Promise<ElectrumResponse>`

Ping the connected Electrum server.

### `getHeader(network: string, height: number): Promise<ElectrumResponse>`

Get a block header at a specific height.

### `getBalance(network: string, scriptHashes: string[]): Promise<ElectrumResponse>`

Get balance for multiple script hashes.

## Types

```typescript
interface Peer {
  host: string;
  ssl?: number;
  tcp?: number;
  protocol?: string;
}

interface StartConfig {
  network: string;
  customPeers?: Peer[];
}

interface ElectrumResponse {
  error: boolean;
  data?: string;
  method?: string;
}
```

## Development

### Prerequisites

- Rust (1.70+)
- Node.js (16+)
- React Native development environment

### Building

```bash
# Install dependencies
npm install

# Build Rust library
npm run build:rust

# Generate bindings
npm run build:bindings

# Build TypeScript
npm run build:ts
```

### Testing

```bash
# Run Rust tests
cargo test

# Run integration tests
npm test
```

## License

MIT

## Credits

Built with [UniFFI](https://mozilla.github.io/uniffi-rs/) for seamless Rust-to-native bindings.
