# ETH Unite DeFi 2025

A cross-chain DeFi application demonstrating atomic swaps and interoperability between different blockchain networks.

## Prerequisites

Before getting started, make sure you have the following installed:

- [Docker](https://docs.docker.com/get-docker/) - Required for running the development environment
- [Stellar CLI](https://stellar.org/developers/tools) - For Stellar network interactions
- [Node.js](https://nodejs.org/) - For running JavaScript/TypeScript components
- [Rust](https://rustup.rs/) - For Rust smart contracts

## Documentation

📖 **Important**: Before proceeding with the setup, please read the comprehensive documentation in the 1inch reference implementation:

**[packages/1inch-ref/README.md](./packages/1inch-ref/README.md)**

## Quick Start

### 1. Install Stellar CLI

```bash
brew install stellar-cli
```

### 2. Start Docker

Make sure Docker is running on your system. You can start it from Docker Desktop or via command line.

### 3. Deploy Full Stack

Run the deployment script to set up the entire development environment:

```bash
./deploy_full_stack.sh
```

## Project Structure

- **`cross-chain-swap/`** - Rust-based smart contracts for cross-chain functionality
- **`foundry/`** - Foundry-based Solidity contracts and testing framework
- **`packages/1inch-ref/`** - Reference implementation of 1inch Network Fusion Atomic Swaps
- **`resolver/`** - Resolver service for handling cross-chain operations

## Development

For detailed development instructions, testing procedures, and deployment guidelines, refer to the individual README files in each package directory.

## Contributing

Please ensure you read the 1inch reference documentation thoroughly before contributing to understand the atomic swap protocol and security considerations.