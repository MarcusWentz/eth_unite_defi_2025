# 1inch Fusion+ Cross-Chain Swap: Ethereum ↔ Stellar

This project extends 1inch's Fusion+ cross-chain swap protocol to enable bidirectional swaps between Ethereum and Stellar, implementing the complete Fusion+ protocol with hashlock and timelock functionality.

## 🎯 Project Overview

**Track**: Extend Fusion+ to Stellar

## ✨ Features

- ✅ **Bidirectional Swaps**: Ethereum ↔ Stellar
- ✅ **Hashlock & Timelock**: Preserved from original Fusion+ protocol
- ✅ **Onchain Execution**: Full token transfers on both chains
- ✅ **1inch Integration**: Uses 1inch cross-chain SDK
- ✅ **Stellar Soroban**: Native smart contract integration
- ✅ **Ethereum Compatibility**: Full EVM support

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   Ethereum      │    │    Stellar      │
│                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │EscrowFactory│ │    │ │EscrowFactory│ │
│ └─────────────┘ │    │ └─────────────┘ │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ BaseEscrow  │ │    │ │ BaseEscrow  │ │
│ └─────────────┘ │    │ └─────────────┘ │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │  Resolver   │ │    │ │  Resolver   │ │
│ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘
         │                       │
         └───────────────────────┘
                    │
         ┌─────────────────────┐
         │  Cross-Chain Client │
         │  (This Project)     │
         └─────────────────────┘
```

## 🚀 Quick Start

### Prerequisites

1. **Deploy Contracts**: Run the full stack deployment
```bash
./deploy_full_stack.sh
```

2. **Install Dependencies**
```bash
cd client
bun install
```

3. **Configure Settings**
Edit `config/config.json` with your:
- Ethereum RPC URL and private key
- Stellar network settings
- Contract addresses from deployment

4. **Run the Demo**
```bash
bun run index.ts
```

## 📁 Project Structure

```
client/
├── index.ts                 # Main demo client
├── cross-chain-swap.ts      # Cross-chain swap logic
├── ethereum-client.ts       # Ethereum integration
├── config/
│   └── config.json         # Configuration
├── bindings/               # Stellar contract bindings
│   ├── resolver/
│   ├── order/
│   └── escrow-factory/
└── README.md
```

## 🔧 Configuration

Update `config/config.json` with your settings:

```json
{
  "ethereum": {
    "rpcUrl": "https://sepolia.infura.io/v3/YOUR_KEY",
    "escrowFactoryAddress": "0x...",
    "privateKey": "0x...",
    "tokens": {
      "usdc": "0x...",
      "weth": "0x..."
    }
  },
  "stellar": {
    "rpcUrl": "http://localhost:8000",
    "tokens": {
      "usdc": "CAPXKPSVXRJ56ZKR6XRA7SB6UGQEZD2UNRO4OP6V2NYTQTV6RFJGIRZM",
      "xlm": "CA7N3TLKV27AYBLL6AR7ICJ6C5AMPMCQOGFKI6ZU2FNHRRDN4CNBL5T5"
    }
  },
  "swapDirection": "ethereum_to_stellar"
}
```

## 🔄 Swap Flow

### Ethereum → Stellar
1. Create source escrow on Ethereum
2. Create destination escrow on Stellar
3. Execute cross-chain transfer
4. Withdraw using hashlock secret

### Stellar → Ethereum
1. Create source escrow on Stellar
2. Create destination escrow on Ethereum
3. Execute cross-chain transfer
4. Withdraw using hashlock secret

## 🔒 Security Features

- **Hashlocks**: Cryptographic commitments for atomic swaps
- **Timelocks**: Time-based security for withdrawals and cancellations
- **Bidirectional**: Swaps work in both directions
- **Atomic**: Either both sides succeed or both fail

## 🛠️ Development

### Running Tests
```bash
bun test
```

### Development Mode
```bash
bun run dev
```

### Building Contracts
```bash
cd ../cross-chain-swap
stellar contract build
```

## 📚 Documentation

- [1inch Fusion+ Protocol](https://1inch.io/assets/1inch-fusion-plus.pdf)
- [Cross-Chain Swap Documentation](https://github.com/1inch/cross-chain-swap/tree/master/documentation)
- [Fusion Protocol Docs](https://github.com/1inch/fusion-protocol/tree/master/docs)

## 🏆 Requirements Met

- ✅ **Preserve hashlock and timelock functionality**
- ✅ **Bidirectional swaps (Ethereum ↔ Stellar)**
- ✅ **Onchain execution of token transfers**
- ✅ **Stretch goal: UI (can be added)**

## 🤝 Contributing

This project demonstrates 1inch Fusion+ protocol extension to Stellar. The implementation follows the original Fusion+ specifications while adapting to Stellar's unique consensus and smart contract model.

## 📄 License

MIT License - see LICENSE file for details.
