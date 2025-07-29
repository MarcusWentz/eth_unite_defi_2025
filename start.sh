#!/bin/zsh

# Load environment and configuration
source .env
source scripts/evm_utils.sh
source scripts/stellar_utils.sh
source scripts/run_stages.sh

CURRENT_DIR=$(dirname "$0")
CONFIG_PATH="${CURRENT_DIR}/packages/1inch-ref/examples/config/config.json"
SCRIPT_PATH="${CURRENT_DIR}/packages/1inch-ref/examples/script/CreateOrder.s.sol:CreateOrder"

DOCKER_IMAGE="stellar/quickstart:latest"
STELLAR_PROJECT_DIR="cross-chain-swap"
SOROBAN_PACKAGE_NAME="cross-chain-swap"
SOROBAN_WASM_NAME="escrow_factory"
STELLAR_IDENTITY_NAME="my-deployer"
DOCKER_CONTAINER_NAME="stellar"
EVM_PROJECT_DIR="packages/1inch-ref"

# Read config values
if [[ -z "$CHAIN_ID" ]]; then
  CHAIN_ID=31337
fi

# Timelocks
WITHDRAWAL_SRC_TIMELOCK=$(jq -r '.withdrawalSrcTimelock' "$CONFIG_PATH")
WITHDRAWAL_DST_TIMELOCK=$(jq -r '.withdrawalDstTimelock' "$CONFIG_PATH")
CANCELLATION_SRC_TIMELOCK=$(jq -r '.cancellationSrcTimelock' "$CONFIG_PATH")
CANCELLATION_DST_TIMELOCK=$(jq -r '.cancellationDstTimelock' "$CONFIG_PATH")

main() {
  echo -n "Do you want to rebuild project? Otherwise it will rebuild automatically if necessary [y/n]: "
  read response
  if [[ "$response" == "y" || "$response" == "Y" ]]; then
    forge clean
    forge build
  fi

  check_stellar_dependencies
  reset_environment
  
  # Start networks
  start_env
  start_stellar_network
  setup_keypair_and_network

  # Build and deploy Stellar contract factory
  deploy_escrow_factory

  # Run stages
  run_stages
  
  # Cleanup
  cleanup_evm
  cleanup_stellar
  
  echo "=== All stages completed ==="
}

# Run main function
main "$@"