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
STELLAR_PROJECT_DIR=".stellar"

SOROBAN_ORDER_NAME="order"      # Matches directory name
SOROBAN_WASM_NAME="order"     # Matches build output name

SOROBAN_DA_NAME="dutch-auction"
SOROBAN_DA_WASM_NAME="dutch_auction"

SOROBAN_RESOLVER_NAME="resolver"
SOROBAN_RESOLVER_WASM_NAME="resolver"

SOROBAN_ESCROW_FACTORY_NAME="escrow-factory"
SOROBAN_ESCROW_FACTORY_WASM_NAME="escrow"

SOROBAN_ESCROW_SRC_NAME="escrow-src"
SOROBAN_ESCROW_SRC_WASM_NAME="escrow_src"
SOROBAN_ESCROW_DST_NAME="escrow-dst"
SOROBAN_ESCROW_DST_WASM_NAME="escrow_dst"

SOROBAN_TEST_TOKEN_NAME="test-token"
SOROBAN_TEST_TOKEN_WASM_NAME="test_token"

XLM_ADDRESS="CAGP76LSLAQ7E274ZTFV7RDFZP42H6HKEDLUQ6IWSADHDHSOG5OGDFT7"

STELLAR_IDENTITY_NAME="my-deployer"
DOCKER_CONTAINER_NAME="stellar"
EVM_PROJECT_DIR="packages/1inch-ref"

ALICE_IDENTITY_NAME="alice"
BOB_IDENTITY_NAME="bob"

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

step "Performing clean reset of environment..."
docker stop ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
docker rm ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
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

# Check if container is already running
if docker ps -q -f "name=${DOCKER_CONTAINER_NAME}" | grep -q .; then
    echo "Docker container '${DOCKER_CONTAINER_NAME}' is already running."
    success "Using existing Stellar network."
else
    # Remove any stopped containers with the same name
    docker rm ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
    
    docker run -d --rm \
      -p 8000:8000 \
      --name ${DOCKER_CONTAINER_NAME} \
      ${DOCKER_IMAGE} \
      --local \
      --enable-soroban-rpc > /dev/null || fail "Failed to start Docker container."

    echo "Waiting for the network to initialize..."
    sleep 30
fi

# Wait for the network to be ready
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

cd cross-chain-swap

step "Building contracts..."

stellar contract build || fail "Cargo build failed."
DA_WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_DA_WASM_NAME}.wasm"

########################################################
# 4. Build and Deploy
step "[1] Building and deploying '${SOROBAN_DA_NAME}' contract..."
echo "Building contract Wasm..."

DA_CONTRACT_ID=$(stellar contract deploy --wasm ${DA_WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias ${SOROBAN_DA_NAME})
[ -z "$DA_CONTRACT_ID" ] && fail "Failed to deploy contract."
success "Contract ${SOROBAN_DA_NAME} deployed! ID: ${DA_CONTRACT_ID}"

step "Generating client bindings..."
stellar contract bindings typescript --wasm ${DA_WASM_PATH} --output-dir ../client/bindings/${SOROBAN_DA_NAME}


########################################################
step "[2] Building and deploying '${SOROBAN_ORDER_NAME}' contract..."
echo "Building contract Wasm..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_WASM_NAME}.wasm"

echo "Deploying contract instance..."
ORDER_MIXIN_ADDRESS=$(stellar contract deploy --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias ${SOROBAN_ORDER_NAME} -- --da_addy ${DA_CONTRACT_ID})
[ -z "$ORDER_MIXIN_ADDRESS" ] && fail "Failed to deploy contract."
success "Contract ${SOROBAN_ORDER_NAME} deployed! ID: ${ORDER_MIXIN_ADDRESS}"

step "Generating client bindings..."
stellar contract bindings typescript --wasm ${WASM_PATH} --output-dir ../client/bindings/${SOROBAN_ORDER_NAME}

########################################################
step "[2.1] Building and deploying test tokens..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_TEST_TOKEN_WASM_NAME}.wasm"

echo "Deploying maker token (USDC)..."
MAKER_TOKEN_ADDRESS=$(stellar contract deploy --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias maker-token -- --admin ${PUBLIC_KEY})
[ -z "$MAKER_TOKEN_ADDRESS" ] && fail "Failed to deploy maker token."
success "Maker token deployed! ID: ${MAKER_TOKEN_ADDRESS}"

echo "Deploying taker token (DAI)..."
TAKER_TOKEN_ADDRESS=$(stellar contract deploy --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias taker-token -- --admin ${PUBLIC_KEY})
[ -z "$TAKER_TOKEN_ADDRESS" ] && fail "Failed to deploy taker token."
success "Taker token deployed! ID: ${TAKER_TOKEN_ADDRESS}"

echo "Minting tokens to deployer account..."
stellar contract invoke --id "${MAKER_TOKEN_ADDRESS}" --source-account ${STELLAR_IDENTITY_NAME} --network local -- mint --admin ${PUBLIC_KEY} --to ${PUBLIC_KEY} --amount 1000000000000000000000
stellar contract invoke --id "${TAKER_TOKEN_ADDRESS}" --source-account ${STELLAR_IDENTITY_NAME} --network local -- mint --admin ${PUBLIC_KEY} --to ${PUBLIC_KEY} --amount 1000000000000000000000

success "Tokens minted to deployer account."

step "Generating test token client bindings..."
stellar contract bindings typescript --wasm ${WASM_PATH} --output-dir ../client/bindings/${SOROBAN_TEST_TOKEN_NAME}

########################################################

step "[3] Building and deploying '${SOROBAN_ESCROW_SRC_NAME}' wasm hash..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_ESCROW_SRC_WASM_NAME}.wasm"

ESCROW_SRC_WASM_HASH=$(stellar contract upload --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local)
[ -z "$ESCROW_SRC_WASM_HASH" ] && fail "Failed to upload contract."
success "Contract ${SOROBAN_ESCROW_SRC_NAME} wasm hash: ${ESCROW_SRC_WASM_HASH}"

########################################################

step "[4] Building and deploying '${SOROBAN_ESCROW_DST_NAME}' wasm hash..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_ESCROW_DST_WASM_NAME}.wasm"

ESCROW_DST_WASM_HASH=$(stellar contract upload --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local)
[ -z "$ESCROW_DST_WASM_HASH" ] && fail "Failed to upload contract."
success "Contract ${SOROBAN_ESCROW_DST_NAME} wasm hash: ${ESCROW_DST_WASM_HASH}"

########################################################
step "[5] Building and deploying '${SOROBAN_ESCROW_FACTORY_NAME}' contract..."
echo "Building contract Wasm..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_ESCROW_FACTORY_WASM_NAME}.wasm"

echo "Deploying contract instance..."
ESCROW_FACTORY_ADDRESS=$(stellar contract deploy --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias ${SOROBAN_ESCROW_FACTORY_NAME} -- --escrow_src_wasm_hash ${ESCROW_SRC_WASM_HASH} --escrow_dst_wasm_hash ${ESCROW_DST_WASM_HASH} --xlm_address ${XLM_ADDRESS})

[ -z "$ESCROW_FACTORY_ADDRESS" ] && fail "Failed to deploy contract."
success "Contract ${SOROBAN_ESCROW_FACTORY_NAME} deployed! ID: ${ESCROW_FACTORY_ADDRESS}"

step "Generating client bindings..."
stellar contract bindings typescript --wasm ${WASM_PATH} --output-dir ../client/bindings/${SOROBAN_ESCROW_FACTORY_NAME}

########################################################
step "[6] Building and deploying '${SOROBAN_RESOLVER_NAME}' contract..."
echo "Building contract Wasm..."

WASM_PATH="./target/wasm32v1-none/release/${SOROBAN_RESOLVER_WASM_NAME}.wasm"

echo "Deploying contract instance..."
CONTRACT_ID=$(stellar contract deploy --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local --alias ${SOROBAN_RESOLVER_NAME} -- --escrow_factory_address ${ESCROW_FACTORY_ADDRESS} --order_mixin_address ${ORDER_MIXIN_ADDRESS})

[ -z "$CONTRACT_ID" ] && fail "Failed to deploy contract."
success "Contract ${SOROBAN_RESOLVER_NAME} deployed! ID: ${CONTRACT_ID}"

step "Generating client bindings..."
stellar contract bindings typescript --wasm ${WASM_PATH} --output-dir ../client/bindings/${SOROBAN_RESOLVER_NAME}

########################################################
step "Updating client configuration..."

cat > ../client/config/config.json << EOF
{
    "escrowFactory": "${ESCROW_FACTORY_ADDRESS}",
    "limitOrderProtocol": "${ORDER_MIXIN_ADDRESS}",
    "deployer": "${PUBLIC_KEY}",
    "maker": "${PUBLIC_KEY}",
    "srcToken": "${MAKER_TOKEN_ADDRESS}",
    "dstToken": "${TAKER_TOKEN_ADDRESS}",
    "resolver": "${CONTRACT_ID}",
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
    ]
}
EOF

success "Client configuration updated with deployed addresses."

########################################################

# stellar keys generate ${ALICE_IDENTITY_NAME} > /dev/null
# ALICE_PUBLIC_KEY=$(stellar keys address ${ALICE_IDENTITY_NAME})
# stellar keys generate ${BOB_IDENTITY_NAME} > /dev/null
# BOB_PUBLIC_KEY=$(stellar keys address ${BOB_IDENTITY_NAME})

# MAKER_ASSET="CAPXKPSVXRJ56ZKR6XRA7SB6UGQEZD2UNRO4OP6V2NYTQTV6RFJGIRZM"
# TAKER_ASSET="CA7N3TLKV27AYBLL6AR7ICJ6C5AMPMCQOGFKI6ZU2FNHRRDN4CNBL5T5"

# MAKER_TRAITS="967101221531144175919556390646195146547200"

# echo "Deploying contract instance..."
# ORDER=$(cat << EOF
# {"maker": "${ALICE_PUBLIC_KEY}", "maker_asset": "${MAKER_ASSET}", "taker_asset": "${TAKER_ASSET}", "maker_traits": "${MAKER_TRAITS}", "receiver": "${BOB_PUBLIC_KEY}", "salt": "1", "taking_amount": "1000000000000000000", "making_amount": "1000000000000000000" }
# EOF
# )

# echo "Invoking 'calculate_making_amount' function..."
# INVOKE_RESULT=$(stellar contract invoke --id "${CONTRACT_ID}" --source-account ${STELLAR_IDENTITY_NAME} --network local -- calculate_making_amount --order '${ORDER}')
# success "Invoke result: ${INVOKE_RESULT}"

# # 5. EVM Stages
# read -p "Stellar setup complete. Proceed with EVM stages? (y/n) " -n 1 -r
# echo
# if [[ $REPLY =~ ^[Yy]$ ]]; then
#     step "Running EVM deployment stages..."
#     # ** CORRECTED SYNTAX: Navigate to the script's directory **
#     cd ${EVM_PROJECT_DIR}/examples/scripts || fail "Could not navigate to the EVM script directory"
    
#     echo "Running EVM deployment script from $(pwd)..."
#     ./create_order.sh
    
#     # Navigate back to the project root
#     cd ../../../..
#     success "EVM stages complete."
# fi

# # 6. Final Cleanup
# step "Cleanup Phase"
# read -p "Do you want to stop the Stellar Docker container? (y/n) " -n 1 -r
# echo
# if [[ $REPLY =~ ^[Yy]$ ]]; then
#     if [ "$(docker ps -q -f "name=${DOCKER_CONTAINER_NAME}")" ]; then
#         echo "Stopping Stellar Docker container..."
#         docker stop ${DOCKER_CONTAINER_NAME} > /dev/null
#         success "Container stopped."
#     fi
# fi

# read -p "Do you want to delete the generated Stellar keys in ~/.config/stellar? (y/n) " -n 1 -r
# echo
# if [[ $REPLY =~ ^[Yy]$ ]]; then
#     echo "Deleting global Stellar config..."
#     rm -rf ~/.config/stellar
#     success "Keys deleted."
# fi

success "All operations finished."