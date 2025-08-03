#!/bin/bash

# 1inch Fusion+ Cross-Chain Swap Demo Orchestrator
# Ethereum ↔ Stellar Integration for Hackathon

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to wait for service to be ready
wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=30
    local attempt=1
    
    print_status "Waiting for $service_name to be ready..."
    
    while [ $attempt -le $max_attempts ]; do
        if curl -s "$url" >/dev/null 2>&1; then
            print_success "$service_name is ready!"
            return 0
        fi
        
        echo -n "."
        sleep 2
        attempt=$((attempt + 1))
    done
    
    print_error "$service_name failed to start after $max_attempts attempts"
    return 1
}

# Function to check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"
    
    local missing_deps=()
    
    # Check for required commands
    if ! command_exists stellar; then
        missing_deps+=("stellar")
    fi
    
    if ! command_exists bun; then
        missing_deps+=("bun")
    fi
    
    if ! command_exists forge; then
        missing_deps+=("forge")
    fi
    
    if ! command_exists docker; then
        missing_deps+=("docker")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing required dependencies: ${missing_deps[*]}"
        print_status "Please install the missing dependencies and try again."
        exit 1
    fi
    
    print_success "All prerequisites are installed!"
}

# Function to start Stellar network
start_stellar_network() {
    print_header "Starting Stellar Network"
    
    # Check if Stellar network is already running
    if curl -s http://localhost:8000/health >/dev/null 2>&1; then
        print_warning "Stellar network appears to be already running"
        return 0
    fi
    
    print_status "Starting Stellar network with Docker..."
    
    # Start Stellar network in background
    docker run --rm -d \
        --name stellar-network \
        -p 8000:8000 \
        -p 11626:11626 \
        stellar/quickstart:latest \
        --standalone \
        --enable-soroban-rpc \
        --protocol-version 20 \
        --history-archive-urls https://history.stellar.org/prd/core-live/core_live_001/ &
    
    # Wait for network to be ready
    wait_for_service "http://localhost:8000/health" "Stellar Network"
}

# Function to deploy Stellar contracts
deploy_stellar_contracts() {
    print_header "Deploying Stellar Contracts"
    
    cd cross-chain-swap
    
    print_status "Building Stellar contracts..."
    stellar contract build || {
        print_error "Failed to build Stellar contracts"
        exit 1
    }
    
    print_status "Running full stack deployment..."
    ../deploy_full_stack.sh || {
        print_error "Failed to deploy Stellar contracts"
        exit 1
    }
    
    cd ..
    print_success "Stellar contracts deployed successfully!"
}

# Function to deploy Ethereum contracts
deploy_ethereum_contracts() {
    print_header "Deploying Ethereum Contracts"
    
    cd packages/1inch-ref
    
    print_status "Building Ethereum contracts..."
    forge build || {
        print_error "Failed to build Ethereum contracts"
        exit 1
    }
    
    # Check if we have environment variables for deployment
    if [ -z "$SEPOLIA_RPC_URL" ] || [ -z "$PRIVATE_KEY" ]; then
        print_warning "Ethereum deployment skipped - missing environment variables"
        print_status "To deploy to Ethereum, set SEPOLIA_RPC_URL and PRIVATE_KEY"
        cd ../..
        return 0
    fi
    
    print_status "Deploying to Sepolia testnet..."
    forge script script/DeployEscrowFactory.s.sol \
        --fork-url "$SEPOLIA_RPC_URL" \
        --private-key "$PRIVATE_KEY" \
        --broadcast \
        --verify || {
        print_error "Failed to deploy Ethereum contracts"
        exit 1
    }
    
    cd ../..
    print_success "Ethereum contracts deployed successfully!"
}

# Function to install client dependencies
install_client_deps() {
    print_header "Installing Client Dependencies"
    
    cd client
    
    print_status "Installing dependencies with Bun..."
    bun install || {
        print_error "Failed to install dependencies"
        exit 1
    }
    
    cd ..
    print_success "Client dependencies installed!"
}

# Function to update configuration
update_config() {
    print_header "Updating Configuration"
    
    cd client
    
    # Check if config needs to be updated with deployed contract addresses
    if [ -f "../cross-chain-swap/deployment_addresses.json" ]; then
        print_status "Updating config with deployed contract addresses..."
        # This would parse the deployment addresses and update config.json
        # For now, we'll use the existing config
    fi
    
    print_warning "Please update client/config/config.json with:"
    print_status "1. Your Ethereum RPC URL and private key"
    print_status "2. Deployed contract addresses"
    print_status "3. Testnet token addresses"
    
    cd ..
}

# Function to run the demo
run_demo() {
    print_header "Running Cross-Chain Swap Demo"
    
    cd client
    
    print_status "Starting 1inch Fusion+ Cross-Chain Swap Demo..."
    print_status "This will demonstrate:"
    print_status "• Bidirectional swaps (Ethereum ↔ Stellar)"
    print_status "• Hashlock and timelock functionality"
    print_status "• Atomic cross-chain transfers"
    print_status "• 1inch Fusion+ protocol integration"
    
    echo ""
    
    # Run the demo
    bun run index.ts || {
        print_error "Demo failed to run"
        exit 1
    }
    
    cd ..
}

# Function to show demo results
show_results() {
    print_header "Demo Results"
    
    print_success "🎉 Cross-Chain Swap Demo Completed!"
    echo ""
    print_status "What was demonstrated:"
    echo "  ✅ Bidirectional cross-chain swaps"
    echo "  ✅ Hashlock and timelock functionality"
    echo "  ✅ 1inch Fusion+ protocol integration"
    echo "  ✅ Stellar and Ethereum interoperability"
    echo "  ✅ Atomic swap execution"
    echo ""
    print_status "Next steps for production:"
    echo "  1. Deploy to mainnet/testnet"
    echo "  2. Add UI interface"
    echo "  3. Implement partial fills"
    echo "  4. Add more token pairs"
}

# Function to cleanup
cleanup() {
    print_header "Cleanup"
    
    print_status "Stopping Stellar network..."
    docker stop stellar-network 2>/dev/null || true
    docker rm stellar-network 2>/dev/null || true
    
    print_success "Cleanup completed!"
}

# Main execution
main() {
    print_header "1inch Fusion+ Cross-Chain Swap Demo"
    print_status "Ethereum ↔ Stellar Integration for Hackathon"
    echo ""
    
    # Check prerequisites
    check_prerequisites
    
    # Start Stellar network
    start_stellar_network
    
    # Deploy contracts
    deploy_stellar_contracts
    deploy_ethereum_contracts
    
    # Install dependencies
    install_client_deps
    
    # Update configuration
    update_config
    
    # Run demo
    run_demo
    
    # Show results
    show_results
    
    # Ask if user wants to cleanup
    echo ""
    read -p "Do you want to stop the Stellar network? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cleanup
    fi
    
    print_success "Demo completed successfully! 🚀"
}

# Handle script interruption
trap 'print_error "Script interrupted. Cleaning up..."; cleanup; exit 1' INT TERM

# Run main function
main "$@" 