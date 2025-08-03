# 1inch Fusion+ Cross-Chain Swap - Setup Guide

## 🚀 Quick Start

### **Option 1: Local Development (Recommended for Testing)**

```bash
# No environment variables needed - everything is automatic
./run_comprehensive_demo.sh
```

### **Option 2: Real Testnet Deployment**

```bash
# Set environment variables for real testnets
export ETH_PRIVATE_KEY="0xYourRealPrivateKeyWithSepoliaETH"
export ETH_RPC_URL="https://sepolia.infura.io/v3/YourProjectId"
export ETHERSCAN_API_KEY="YourEtherscanApiKey"

# Run the demo
./run_comprehensive_demo.sh
```

## 📋 Prerequisites

### **Required Tools**
- **Docker** - For Stellar network
- **Cargo** - For Rust contract compilation
- **Forge** (Foundry) - For Ethereum contract compilation
- **Bun** - For TypeScript client
- **Curl** - For network testing
- **Soroban CLI** - For Stellar contract deployment
- **Anvil** (Foundry) - For local Ethereum (auto-installed with Foundry)

### **Installation Commands**

```bash
# Install Foundry (includes forge and anvil)
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Install Bun
curl -fsSL https://bun.sh/install | bash

# Install Soroban CLI
curl -sSf https://soroban.stellar.org | sh

# Install Docker (if not already installed)
# macOS: brew install --cask docker
# Ubuntu: sudo apt-get install docker.io
```

## 🔧 Setup Scenarios

### **Scenario 1: Local Development (Zero Setup)**

**Perfect for:**
- ✅ Testing and development
- ✅ Hackathon demos
- ✅ No real funds needed
- ✅ Instant setup

**What happens automatically:**
1. **Anvil** starts on port 8545 (local Ethereum)
2. **Stellar** starts in Docker on port 8000 (local Stellar)
3. **Contracts** deploy to local networks
4. **Demo** runs with real transactions on local networks
5. **Everything** is REAL but local

**Command:**
```bash
./run_comprehensive_demo.sh
```

**No environment variables needed!** 🎉

### **Scenario 2: Real Testnet Deployment**

**Perfect for:**
- ✅ Production testing
- ✅ Real blockchain interaction
- ✅ Hackathon submissions
- ✅ Live demonstrations

**Required Setup:**
```bash
# Get Sepolia ETH from faucet
# https://sepoliafaucet.com/

# Get Infura/Alchemy RPC URL
# https://infura.io/ or https://alchemy.com/

# Get Etherscan API key
# https://etherscan.io/apis

# Set environment variables
export ETH_PRIVATE_KEY="0xYourRealPrivateKeyWithSepoliaETH"
export ETH_RPC_URL="https://sepolia.infura.io/v3/YourProjectId"
export ETHERSCAN_API_KEY="YourEtherscanApiKey"
```

**Command:**
```bash
./run_comprehensive_demo.sh
```

## 💰 Funding Requirements

### **Local Development**
- ✅ **No funding needed** - Anvil provides unlimited ETH
- ✅ **No funding needed** - Friendbot funds Stellar accounts automatically

### **Real Testnets**
- ✅ **Sepolia ETH** - ~0.01 ETH for gas fees (get from faucet)
- ✅ **Stellar accounts** - Auto-funded by friendbot

## 🔍 What the Script Does

### **Automatic Detection**
The script automatically detects your setup:

```bash
# If ETH_RPC_URL contains "localhost" or "127.0.0.1"
→ LOCAL MODE (Anvil + Local Stellar)

# If ETH_RPC_URL is external (Infura, Alchemy, etc.)
→ TESTNET MODE (Sepolia + Stellar Testnet)
```

### **Local Mode Features**
- ✅ Starts Anvil automatically
- ✅ Starts Stellar Docker container
- ✅ Uses default Anvil private key
- ✅ Auto-funds Stellar accounts
- ✅ Deploys all contracts locally
- ✅ Runs complete demo

### **Testnet Mode Features**
- ✅ Validates real credentials
- ✅ Deploys to Sepolia testnet
- ✅ Deploys to Stellar testnet
- ✅ Verifies contracts on Etherscan
- ✅ Runs complete demo with real networks

## 🛠️ Troubleshooting

### **Common Issues**

**"Missing dependencies"**
```bash
# Install missing tools
curl -L https://foundry.paradigm.xyz | bash
foundryup
curl -fsSL https://bun.sh/install | bash
curl -sSf https://soroban.stellar.org | sh
```

**"Docker not running"**
```bash
# Start Docker
# macOS: Open Docker Desktop
# Linux: sudo systemctl start docker
```

**"Anvil not found"**
```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

**"Insufficient funds" (Testnet)**
```bash
# Get Sepolia ETH
# Visit: https://sepoliafaucet.com/
# Or: https://sepolia.infura.io/v3/faucet
```

### **Network Issues**

**Stellar connection failed**
```bash
# Check if Stellar is running
curl http://localhost:8000/soroban/rpc/v1/health

# Restart Stellar
docker stop stellar && docker rm stellar
docker run -d --name stellar -p 8000:8000 stellar/quickstart:latest --local --enable-soroban-rpc
```

**Ethereum connection failed**
```bash
# Check if Anvil is running
curl http://localhost:8545

# Start Anvil
anvil --port 8545
```

## 🎯 Success Criteria

The setup is successful when:

### **Local Development**
- ✅ Anvil running on port 8545
- ✅ Stellar running on port 8000
- ✅ All contracts deployed locally
- ✅ Demo executes successfully
- ✅ All tests pass

### **Testnet Deployment**
- ✅ Contracts deployed to Sepolia
- ✅ Contracts deployed to Stellar testnet
- ✅ Contracts verified on Etherscan
- ✅ Demo executes successfully
- ✅ All tests pass

## 🚀 Ready to Deploy!

### **For Local Development:**
```bash
./run_comprehensive_demo.sh
```

### **For Testnet Deployment:**
```bash
export ETH_PRIVATE_KEY="0xYourKey"
export ETH_RPC_URL="https://sepolia.infura.io/v3/YourProjectId"
export ETHERSCAN_API_KEY="YourKey"

./run_comprehensive_demo.sh
```

**That's it! Everything else is automatic.** 🎉 