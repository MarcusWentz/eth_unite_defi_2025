start_stellar_network() {
  echo "Starting local Stellar network..."
  docker run -d --rm \
    -p 8000:8000 \
    --name ${DOCKER_CONTAINER_NAME} \
    ${DOCKER_IMAGE} \
    --local \
    --enable-soroban-rpc > /dev/null || fail "Failed to start Docker container."

  echo "Waiting for the network to initialize..."
  sleep 25
  until curl -s -f -o /dev/null http://localhost:8000/; do
    echo -n "."
    sleep 2
  done
  
  echo "Local Stellar network is online."
}

check_stellar_dependencies() {
  echo "Checking for dependencies..."
  command -v docker &> /dev/null || fail "Docker is not installed."
  command -v cargo &> /dev/null || fail "Cargo (Rust) is not installed."
  command -v stellar &> /dev/null || fail "Stellar CLI is not installed (e.g., via brew)."
  echo "All dependencies are installed."
}

reset_environment() {
  echo "Performing clean reset of environment..."
  docker stop ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
  rm -rf ~/.config/stellar
  rm -rf ./.stellar
  rm -rf ${STELLAR_PROJECT_DIR}/.config
  echo "Environment reset."
}

setup_keypair_and_network() {
  echo "Setting up Stellar identity and network config..."

  echo "Generating keypair for '${STELLAR_IDENTITY_NAME}'..."
  stellar keys generate ${STELLAR_IDENTITY_NAME} > /dev/null
  PUBLIC_KEY=$(stellar keys address ${STELLAR_IDENTITY_NAME})
  echo "Using identity '${STELLAR_IDENTITY_NAME}' with Public Key: ${PUBLIC_KEY}"

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
    echo "Account successfully funded."
  else
    echo "Account is already funded."
  fi
}

deploy_escrow_factory() {
  echo "Building and deploying '${SOROBAN_PACKAGE_NAME}' contract..."
  cargo build --manifest-path ./${STELLAR_PROJECT_DIR}/contracts/${SOROBAN_PACKAGE_NAME}/Cargo.toml --target wasm32-unknown-unknown --release || fail "Cargo build failed."
  WASM_PATH="./${STELLAR_PROJECT_DIR}/target/wasm32-unknown-unknown/release/${SOROBAN_WASM_NAME}.wasm"

  echo "Uploading Wasm..."
  WASM_HASH=$(stellar contract upload --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network local)
  [ -z "$WASM_HASH" ] && fail "Failed to upload Wasm."
  echo "Wasm uploaded. Hash: ${WASM_HASH}"

  echo "Deploying contract instance..."
  CONTRACT_ID=$(stellar contract deploy --wasm-hash "${WASM_HASH}" --source-account ${STELLAR_IDENTITY_NAME} --network local)
  [ -z "$CONTRACT_ID" ] && fail "Failed to deploy contract."
  echo "Contract deployed! ID: ${CONTRACT_ID}"
}

cleanup_stellar() {  
  echo -n "Stop Stellar container? [y/n]: "
  read answer
  if [[ "$answer" == "y" || "$answer" == "Y" ]]; then
    echo "Stopping Stellar container..."
    docker stop ${DOCKER_CONTAINER_NAME} > /dev/null 2>&1
    echo "Stellar container stopped."
  fi
}

deployEscrowDstStellar() {
  echo "Invoking 'create_dst_escrow' function..."

  INVOKE_RESULT=$(stellar contract invoke \
    --id "${CONTRACT_ID}" \
  --source-account ${STELLAR_IDENTITY_NAME} \
  --network local \
  -- \
  create_dst_escrow \
    --dst_immutables '{
    "order_hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    "hashlock": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    "maker": "GAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "taker": "GBXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", 
    "token": "GCXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "amount": "1000000",
    "safety_deposit": "100000",
    "timelocks": "encoded_timelocks_value"
  }' \
  --src_cancellation_timestamp "1735689600000"
  )

  echo "Invoke result: ${INVOKE_RESULT}"
}