#!/bin/bash

# 1inch Fusion+ Cross-Chain Swap Demo - Stellar Focus
# Simplified version for hackathon demonstration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}================================${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${PURPLE}================================${NC}"
}

# Check if Stellar network is running
check_stellar_network() {
    print_header "Checking Stellar Network"
    
    if curl -s http://localhost:8000/health >/dev/null 2>&1; then
        print_success "Stellar network is running!"
        return 0
    else
        print_warning "Stellar network not running. Starting it..."
        start_stellar_network
    fi
}

# Start Stellar network
start_stellar_network() {
    print_status "Starting Stellar network with Docker..."
    
    # Stop any existing container
    docker stop stellar-network 2>/dev/null || true
    docker rm stellar-network 2>/dev/null || true
    
    # Start new container
    docker run --rm -d \
        --name stellar-network \
        -p 8000:8000 \
        -p 11626:11626 \
        stellar/quickstart:latest \
        --standalone \
        --enable-soroban-rpc \
        --protocol-version 20 &
    
    # Wait for network to be ready
    print_status "Waiting for Stellar network to be ready..."
    for i in {1..30}; do
        if curl -s http://localhost:8000/health >/dev/null 2>&1; then
            print_success "Stellar network is ready!"
            return 0
        fi
        echo -n "."
        sleep 2
    done
    
    print_error "Stellar network failed to start"
    exit 1
}

# Deploy Stellar contracts
deploy_stellar_contracts() {
    print_header "Deploying Stellar Contracts"
    
    cd cross-chain-swap
    
    print_status "Building contracts..."
    stellar contract build || {
        print_error "Failed to build contracts"
        exit 1
    }
    
    print_status "Deploying contracts..."
    ../deploy_full_stack.sh || {
        print_error "Failed to deploy contracts"
        exit 1
    }
    
    cd ..
    print_success "Stellar contracts deployed!"
}

# Install client dependencies
install_client_deps() {
    print_header "Installing Client Dependencies"
    
    cd client
    
    print_status "Installing dependencies..."
    bun install || {
        print_error "Failed to install dependencies"
        exit 1
    }
    
    cd ..
    print_success "Dependencies installed!"
}

# Update configuration for Stellar-only demo
update_config_for_demo() {
    print_header "Updating Configuration for Demo"
    
    cd client
    
    # Create a demo config that focuses on Stellar side
    cat > config/config.json << 'EOF'
{
    "escrowFactory": "0xa7bCb4EAc8964306F9e3764f67Db6A7af6DdF99A",
    "limitOrderProtocol": "CC3KUIKQ6FV3IQUPB6ZAL5VDOBNKVI4AZ22XALK5MGEWAF4WBZH7HS3Y",
    "deployer": "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
    "maker": "0x70997970C51812dc3A010C7d01b50e0d17dc79C8",
    "srcToken": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "dstToken": "CAGP76LSLAQ7E274ZTFV7RDFZP42H6HKEDLUQ6IWSADHDHSOG5OGDFT7",
    "resolver": "CDPH54LWN2HHXPR2GYVWON2E6IXSQKWSPFC3VODS73SSEUYKAHBI4RLH",
    "srcAmount": 100,
    "dstAmount": 300,
    "safetyDeposit": 1,
    "withdrawalSrcTimelock": 300,
    "publicWithdrawalSrcTimelock": 600,
    "cancellationSrcTimelock": 900,
    "publicCancellationSrcTimelock": 1200,
    "withdrawalDstTimelock": 300,
    "publicWithdrawalDstTimelock": 600,
    "cancellationDstTimelock": 900,
    "secret": "secret1",
    "stages": [
        "deployMocks",
        "deployEscrowSrc",
        "deployEscrowDst",
        "withdrawSrc",
        "withdrawDst"
    ],
    "ethereum": {
        "rpcUrl": "https://sepolia.infura.io/v3/demo",
        "escrowFactoryAddress": "0x1234567890123456789012345678901234567890",
        "privateKey": "0x1234567890123456789012345678901234567890123456789012345678901234",
        "chainId": 11155111,
        "tokens": {
            "usdc": "0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238",
            "weth": "0x7b79995e5f793A07Bc00c21412e50Ecae098E7f9"
        }
    },
    "stellar": {
        "rpcUrl": "http://localhost:8000",
        "networkPassphrase": "Standalone Network ; February 2017",
        "tokens": {
            "usdc": "CAPXKPSVXRJ56ZKR6XRA7SB6UGQEZD2UNRO4OP6V2NYTQTV6RFJGIRZM",
            "xlm": "CA7N3TLKV27AYBLL6AR7ICJ6C5AMPMCQOGFKI6ZU2FNHRRDN4CNBL5T5"
        }
    },
    "swapDirection": "stellar_demo"
}
EOF
    
    print_success "Configuration updated for Stellar demo!"
    cd ..
}

# Run the demo
run_demo() {
    print_header "Running Stellar Cross-Chain Swap Demo"
    
    cd client
    
    print_status "Starting 1inch Fusion+ Cross-Chain Swap Demo..."
    print_status "This will demonstrate:"
    print_status "• Stellar smart contract deployment"
    print_status "• Hashlock and timelock functionality"
    print_status "• Order creation and signing"
    print_status "• Cross-chain swap preparation"
    print_status "• 1inch Fusion+ protocol integration"
    
    echo ""
    
    # Run the demo
    bun run index.ts || {
        print_error "Demo failed to run"
        exit 1
    }
    
    cd ..
}

# Show results
show_results() {
    print_header "Demo Results"
    
    print_success "🎉 Stellar Cross-Chain Swap Demo Completed!"
    echo ""
    print_status "What was demonstrated:"
    echo "  ✅ Stellar smart contract deployment"
    echo "  ✅ Hashlock and timelock functionality"
    echo "  ✅ Order creation and signing"
    echo "  ✅ Cross-chain swap preparation"
    echo "  ✅ 1inch Fusion+ protocol integration"
    echo ""
    print_status "This demonstrates the core Fusion+ protocol:"
    echo "  • Cryptographic hashlocks for atomic swaps"
    echo "  • Time-based security with timelocks"
    echo "  • Bidirectional swap capability"
    echo "  • Stellar Soroban smart contract integration"
    echo ""
    print_status "For full bidirectional demo:"
    echo "  1. Deploy Ethereum contracts"
    echo "  2. Configure real RPC URLs and private keys"
    echo "  3. Run the complete cross-chain swap"
}

# Main execution
main() {
    print_header "1inch Fusion+ Cross-Chain Swap Demo"
    print_status "Stellar Focus - Hackathon Demonstration"
    echo ""
    
    # Check and start Stellar network
    check_stellar_network
    
    # Deploy contracts
    deploy_stellar_contracts
    
    # Install dependencies
    install_client_deps
    
    # Update configuration
    update_config_for_demo
    
    # Run demo
    run_demo
    
    # Show results
    show_results
    
    print_success "Demo completed successfully! 🚀"
}

# Handle script interruption
trap 'print_error "Script interrupted. Cleaning up..."; exit 1' INT TERM

# Run main function
main "$@" 