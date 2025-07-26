#!/bin/bash

# ==============================================================================
# Final Full-Stack Development Environment Script (v7 - Final)
# ==============================================================================
# This version uses the correct stellar-cli syntax for all commands,
# ensures a full environment reset, and correctly navigates to the EVM
# script directory to ensure it finds its .env file.
# ==============================================================================

# --- Configuration ---
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

DOCKER_IMAGE="stellar/quickstart:latest"
STELLAR_PROJECT_DIR="stellar"
SOROBAN_PACKAGE_NAME="hello_world" # Matches directory name
SOROBAN_WASM_NAME="hello_world"     # Matches build output name
STELLAR_IDENTITY_NAME="my-deployer"
DOCKER_CONTAINER_NAME="stellar"
EVM_PROJECT_DIR="packages/1inch-ref"

# --- Helper Functions ---
step() { echo -e "\n${YELLOW}STEP: $1${NC}"; }
success() { echo -e "${GREEN}✅ $1${NC}"; }
fail() {
  echo -e "\n${RED}❌ ERROR: $1${NC}"
  echo -e "${YELLOW}The script has failed. The Docker container '${DOCKER_CONTAINER_NAME}' is still running for debugging purposes.${NC}"
  echo -e "${YELLOW}To see the logs, run: docker logs ${DOCKER_CONTAINER_NAME}${NC}"
  echo -e "${YELLOW}To stop the container, run: docker stop ${DOCKER_CONTAINER_NAME}${NC}"
  exit 1
}

# --- Main Logic ---

# Perform a full reset first to guarantee a clean slate
step "Performing clean reset of environment..."
docker stop ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
# Delete ALL known key storage locations
rm -rf ~/.config/stellar
rm -rf ./.stellar
rm -rf ${STELLAR_PROJECT_DIR}/.config
success "Environment reset."

# 1. Check Dependencies
step "Checking for dependencies..."
command -v docker &> /dev/null || fail "Docker is not installed."
command -v cargo &> /dev/null || fail "Cargo (Rust) is not installed."
command -v stellar &> /dev/null || fail "Stellar CLI is not installed (e.g., via brew)."
success "All dependencies are installed."

# 2. Start Stellar Network
step "Starting local Stellar network..."
docker run -d --rm \
  -p 8000:8000 \
  --name ${DOCKER_CONTAINER_NAME} \
  ${DOCKER_IMAGE} \
  --local \
  --enable-soroban-rpc > /dev/null || fail "Failed to start Docker container."

echo "Waiting for the network to initialize..."
sleep 45
until curl -s -f -o /dev/null http://localhost:8000/; do
  echo -n "."
  sleep 2
done
echo ""
success "Local Stellar network is online."

# 3. Setup Identity and Network Config
step "Setting up Stellar identity and network config..."

echo "Generating keypair for '${STELLAR_IDENTITY_NAME}'..."
stellar keys generate ${STELLAR_IDENTITY_NAME} > /dev/null
PUBLIC_KEY=$(stellar keys address ${STELLAR_IDENTITY_NAME})
success "Using identity '${STELLAR_IDENTITY_NAME}' with Public Key: ${PUBLIC_KEY}"

# ** CORRECTED SYNTAX: Configure the 'local' network profile **
echo "Configuring 'local' network profile..."
stellar network add \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Standalone Network ; February 2017" \
  "local" > /dev/null

if ! curl -s http://localhost:8000/accounts/${PUBLIC_KEY} | grep '"balance": "10000.0000000"' > /dev/null; then
  echo "Funding account ${PUBLIC_KEY} with Friendbot..."
  curl -s "http://localhost:8000/friendbot?addr=${PUBLIC_KEY}" > /dev/null
  sleep 5
  if ! curl -s http://localhost:8000/accounts/${PUBLIC_KEY} | grep '"balance": "10000.0000000"' > /dev/null; then
    fail "Failed to fund account. Check Docker logs."
  fi
  success "Account successfully funded."
else
  success "Account is already funded."
fi

# 4. Build and Deploy
step "Building and deploying '${SOROBAN_PACKAGE_NAME}' contract..."
echo "Building contract Wasm..."
cargo build --manifest-path ./${STELLAR_PROJECT_DIR}/contracts/${SOROBAN_PACKAGE_NAME}/Cargo.toml --target wasm32-unknown-unknown --release || fail "Cargo build failed."
WASM_PATH="./${STELLAR_PROJECT_DIR}/target/wasm32-unknown-unknown/release/${SOROBAN_WASM_NAME}.wasm"

# ** CORRECTED SYNTAX: Use --network local **
echo "Uploading Wasm..."
WASM_HASH=$(stellar contract upload --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local)
[ -z "$WASM_HASH" ] && fail "Failed to upload Wasm."
success "Wasm uploaded. Hash: ${WASM_HASH}"

echo "Deploying contract instance..."
CONTRACT_ID=$(stellar contract deploy --wasm-hash "${WASM_HASH}" --source-account ${STELLAR_IDENTITY_NAME} --network local)
[ -z "$CONTRACT_ID" ] && fail "Failed to deploy contract."
success "Contract deployed! ID: ${CONTRACT_ID}"

echo "Invoking 'hello' function..."
INVOKE_RESULT=$(stellar contract invoke --id "${CONTRACT_ID}" --source-account ${STELLAR_IDENTITY_NAME} --network local -- hello --to "Automated Deploy")
success "Invoke result: ${INVOKE_RESULT}"

# 5. EVM Stages
read -p "Stellar setup complete. Proceed with EVM stages? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    step "Running EVM deployment stages..."
    # ** CORRECTED SYNTAX: Navigate to the script's directory **
    cd ${EVM_PROJECT_DIR}/examples/scripts || fail "Could not navigate to the EVM script directory"
    
    echo "Running EVM deployment script from $(pwd)..."
    ./create_order.sh
    
    # Navigate back to the project root
    cd ../../../..
    success "EVM stages complete."
fi

# 6. Final Cleanup
step "Cleanup Phase"
read -p "Do you want to stop the Stellar Docker container? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ "$(docker ps -q -f "name=${DOCKER_CONTAINER_NAME}")" ]; then
        echo "Stopping Stellar Docker container..."
        docker stop ${DOCKER_CONTAINER_NAME} > /dev/null
        success "Container stopped."
    fi
fi

read -p "Do you want to delete the generated Stellar keys in ~/.config/stellar? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Deleting global Stellar config..."
    rm -rf ~/.config/stellar
    success "Keys deleted."
fi

success "All operations finished."