evm_start() {
  # Start anvil if local
  if [[ "$CHAIN_ID" == "31337" ]]; then
    echo "Launching anvil with fork from $RPC_URL and block-time 1..."
    anvil --fork-url "$RPC_URL" --block-time 1 --chain-id 31337 --steps-tracing --host 127.0.0.1 --port 8545 -vvvvv > anvil.log 2>&1 &
    ANVIL_PID=$!
    sleep 15
    # Get anvil start timestamp
    ANVIL_START_TIMESTAMP=$(cast block latest --rpc-url http://localhost:8545 | grep timestamp | awk '{print $2}')
    echo "Anvil start timestamp: $ANVIL_START_TIMESTAMP"
    RPC_URL="http://localhost:8545"
  fi

  # Read stages array from config.json
  STAGES=($(jq -r '.stages[]' "$CONFIG_PATH"))

  ANVIL_DEPLOY_SRC_TIMESTAMP=$ANVIL_START_TIMESTAMP
  ANVIL_DEPLOY_DST_TIMESTAMP=$ANVIL_START_TIMESTAMP
  TIME_DELTA=5
}

evm_cleanup() {
  if [[ "$CHAIN_ID" == "31337" ]]; then
    echo -n "Cleanup anvil instance? [y/n]:" 
    read answer

    if [[ "$answer" == "y" || "$answer" == "Y" ]]; then
        echo "Killing anvil..."
        kill $ANVIL_PID
    else
        echo "Don't forget to kill anvil manually by running 'kill $ANVIL_PID' if you want to stop it."
    fi
  fi
}