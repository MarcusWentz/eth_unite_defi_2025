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
- ✅ **One-Command Demo**: Complete setup and demonstration

## 🚀 Quick Start

### Option 1: Stellar-Focused Demo (Recommended)

```bash
# Run the complete demo with one command
./run_stellar_demo.sh
```

This will:
1. Start Stellar network
2. Deploy Stellar contracts
3. Install dependencies
4. Run the cross-chain swap demo
5. Show results

### Option 2: Full Bidirectional Demo

```bash
# Set up environment variables
export SEPOLIA_RPC_URL="https://sepolia.infura.io/v3/YOUR_KEY"
export PRIVATE_KEY="0xYOUR_PRIVATE_KEY"

# Run the complete demo
./run_demo.sh
```

### Option 3: Manual Setup

```bash
# 1. Start Stellar network
docker run --rm -d --name stellar-network -p 8000:8000 -p 11626:11626 \
  stellar/quickstart:latest --standalone --enable-soroban-rpc --protocol-version 20

# 2. Deploy contracts
./deploy_full_stack.sh

# 3. Install client dependencies
cd client && bun install

# 4. Run demo
bun run index.ts
```

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

## 📁 Project Structure

```
eth_unite_defi_2025/
├── run_demo.sh              # Complete demo orchestrator
├── run_stellar_demo.sh      # Stellar-focused demo
├── deploy_full_stack.sh     # Stellar contract deployment
├── cross-chain-swap/        # Stellar smart contracts
│   ├── contracts/
│   │   ├── escrow/
│   │   ├── resolver/
│   │   └── order/
│   └── src/
├── packages/1inch-ref/      # Ethereum contracts
│   ├── contracts/
│   ├── script/
│   └── test/
├── client/                  # Cross-chain client
│   ├── index.ts            # Main demo client
│   ├── cross-chain-swap.ts # Cross-chain logic
│   ├── ethereum-client.ts  # Ethereum integration
│   ├── config/
│   └── bindings/
└── README.md
```

## 🔧 Configuration

### Stellar Configuration
```json
{
  "stellar": {
    "rpcUrl": "http://localhost:8000",
    "networkPassphrase": "Standalone Network ; February 2017",
    "tokens": {
      "usdc": "CAPXKPSVXRJ56ZKR6XRA7SB6UGQEZD2UNRO4OP6V2NYTQTV6RFJGIRZM",
      "xlm": "CA7N3TLKV27AYBLL6AR7ICJ6C5AMPMCQOGFKI6ZU2FNHRRDN4CNBL5T5"
    }
  }
}
```

### Ethereum Configuration
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
  }
}
```

## 🔄 Fusion+ Protocol Implementation

### Hashlocks
- Cryptographic commitments for atomic swaps
- Ensures both sides must complete or both fail
- Uses keccak256 for cross-chain compatibility

### Timelocks
- Time-based security for withdrawals and cancellations
- Prevents indefinite locking of funds
- Configurable time windows for different operations

### Bidirectional Flow
1. **Source Escrow Creation**: Lock funds on source chain
2. **Destination Escrow Creation**: Lock funds on destination chain
3. **Secret Revelation**: Unlock funds using hashlock secret
4. **Atomic Completion**: Both sides succeed or both fail

## 🛠️ Development

### Prerequisites
- Docker
- Stellar CLI
- Bun
- Foundry (for Ethereum contracts)

### Building Contracts
```bash
# Stellar contracts
cd cross-chain-swap
stellar contract build

# Ethereum contracts
cd packages/1inch-ref
forge build
```

### Running Tests
```bash
# Stellar tests
cd cross-chain-swap
cargo test

# Ethereum tests
cd packages/1inch-ref
forge test
```

## 📚 Documentation

- [1inch Fusion+ Protocol](https://1inch.io/assets/1inch-fusion-plus.pdf)
- [Cross-Chain Swap Documentation](https://github.com/1inch/cross-chain-swap/tree/master/documentation)
- [Fusion Protocol Docs](https://github.com/1inch/fusion-protocol/tree/master/docs)

## 🏆 Requirements Met

- ✅ **Preserve hashlock and timelock functionality**
- ✅ **Bidirectional swaps (Ethereum ↔ Stellar)**
- ✅ **Onchain execution of token transfers**
- ✅ **Stellar Soroban smart contract integration**
- ✅ **1inch Fusion+ protocol compliance**

## 🎯 Demo Scripts

### `run_stellar_demo.sh`
Perfect for demonstrations:
- Focuses on Stellar side (no Ethereum setup required)
- Demonstrates core Fusion+ protocol components
- Shows hashlock and timelock functionality
- Ready for Ethereum integration

### `run_demo.sh`
Complete bidirectional demo:
- Requires Ethereum configuration
- Full cross-chain swap demonstration
- Production-ready setup

## 🔒 Security Features

- **Hashlocks**: Cryptographic commitments for atomic swaps
- **Timelocks**: Time-based security for withdrawals and cancellations
- **Bidirectional**: Swaps work in both directions
- **Atomic**: Either both sides succeed or both fail
- **Audited**: Based on 1inch's audited Fusion+ protocol

## 🤝 Contributing

This project demonstrates 1inch Fusion+ protocol extension to Stellar. The implementation follows the original Fusion+ specifications while adapting to Stellar's unique consensus and smart contract model.

## 📄 License

MIT License - see LICENSE file for details.

---

**Ready for Demo! 🚀**

Run `./run_stellar_demo.sh` to see the complete Fusion+ protocol in action!