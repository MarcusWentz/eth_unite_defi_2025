stellar_check_dependencies() {
  echo "Checking for dependencies..."
  command -v cargo &> /dev/null || fail "Cargo (Rust) is not installed."
  command -v stellar &> /dev/null || fail "Stellar CLI is not installed (e.g., via brew)."
  echo "All dependencies are installed."
}

stellar_setup_keypair() {
  echo "Setting up Stellar identity and network config..."

  echo "Generating keypair for '${STELLAR_IDENTITY_NAME}'..."
  stellar keys generate ${STELLAR_IDENTITY_NAME} > /dev/null
  PUBLIC_KEY=$(stellar keys address ${STELLAR_IDENTITY_NAME})
  echo "Using identity '${STELLAR_IDENTITY_NAME}' with Public Key: ${PUBLIC_KEY}"

  echo "Configuring 'testnet' network profile..."
  stellar network add \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase "Test SDF Network ; September 2015" \
    "testnet" > /dev/null

  echo "Public Key: ${PUBLIC_KEY}"
  
  # Check if account exists first
  echo "Checking account status..."
  ACCOUNT_STATUS=$(curl -s "https://horizon-testnet.stellar.org/accounts/${PUBLIC_KEY}" | jq -r '.status // "not_found"')
  
  if [[ "$ACCOUNT_STATUS" == "not_found" ]] || [[ "$ACCOUNT_STATUS" == "404" ]]; then
    echo "Account not found. Funding with Friendbot..."
    FRIENDBOT_RESPONSE=$(curl -s "https://friendbot.stellar.org?addr=${PUBLIC_KEY}")
    echo "Friendbot response: ${FRIENDBOT_RESPONSE}"
    
    # Wait for account creation
    sleep 10
    
    # Verify account was created
    ACCOUNT_CHECK=$(curl -s "https://horizon-testnet.stellar.org/accounts/${PUBLIC_KEY}" | jq -r '.account_id // "not_found"')
    
    if [[ "$ACCOUNT_CHECK" == "not_found" ]]; then
      fail "Failed to create account via Friendbot"
    fi
    
    echo "Account successfully created and funded."
  else
    echo "Account already exists."
  fi
  
  # Get current balance
  BALANCE=$(curl -s "https://horizon-testnet.stellar.org/accounts/${PUBLIC_KEY}" | jq -r '.balances[] | select(.asset_type=="native") | .balance')
  echo "Current XLM balance: ${BALANCE}"
}

stellar_deploy_escrow_factory() {
  echo "Building and deploying '${SOROBAN_PACKAGE_NAME}' contract..."
  cargo build --manifest-path ./${STELLAR_PROJECT_DIR}/contracts/${SOROBAN_PACKAGE_NAME}/Cargo.toml --target wasm32-unknown-unknown --release || fail "Cargo build failed."
  WASM_PATH="./${STELLAR_PROJECT_DIR}/target/wasm32-unknown-unknown/release/${SOROBAN_WASM_NAME}.wasm"

  echo "Uploading Wasm..."
  WASM_HASH=$(stellar contract upload --wasm ${WASM_PATH} --source-account ${STELLAR_IDENTITY_NAME} --network testnet)
  [ -z "$WASM_HASH" ] && fail "Failed to upload Wasm."
  echo "Wasm uploaded. Hash: ${WASM_HASH}"

  echo "Deploying contract instance..."
  CONTRACT_ID=$(stellar contract deploy --wasm-hash "${WASM_HASH}" --source-account ${STELLAR_IDENTITY_NAME} --network testnet)
  [ -z "$CONTRACT_ID" ] && fail "Failed to deploy contract."
  echo "Contract deployed! ID: ${CONTRACT_ID}"
}

deployEscrowDstStellar() {
  echo "Invoking 'create_dst_escrow' function..."

  # Maker is private key deploying everything
  # Taker is the address of the swap funder

  echo "---- ID: ${CONTRACT_ID} ----"
  echo "---- STELLAR_IDENTITY_NAME: ${STELLAR_IDENTITY_NAME} ----"

  INVOKE_RESULT=$(stellar contract invoke \
  --id "${CONTRACT_ID}" \
  --source-account ${STELLAR_IDENTITY_NAME} \
  --network testnet \
  -- \
  create_dst_escrow \
    --dst_immutables '{
    "order_hash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
    "hashlock": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
    "maker": "{PUBLIC_KEY},
    "taker": {"PUBLIC_KEY}", 
    "token": "${DST_TOKEN}",
    "amount": "${DST_AMOUNT}",
    "safety_deposit": "${SAFETY_DEPOSIT}",
    "timelocks": "encoded_timelocks_value"
    }' \
  )

  echo "Invoke result: ${INVOKE_RESULT}"
}