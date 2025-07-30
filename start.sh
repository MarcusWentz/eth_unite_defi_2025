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
SOROBAN_WASM_NAME="cross_chain_swap"
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

# Config values
SRC_TOKEN=$(jq -r '.srcToken' "$CONFIG_PATH")
DST_TOKEN=$(jq -r '.dstToken' "$CONFIG_PATH")
SRC_AMOUNT=$(jq -r '.srcAmount' "$CONFIG_PATH")
DST_AMOUNT=$(jq -r '.dstAmount' "$CONFIG_PATH")
SAFETY_DEPOSIT=$(jq -r '.safetyDeposit' "$CONFIG_PATH")
TIME_DELTA=5

main() {
  echo -n "Do you want to rebuild project? Otherwise it will rebuild automatically if necessary [y/n]: "
  read response
  if [[ "$response" == "y" || "$response" == "Y" ]]; then
    forge clean
    forge build
  fi

  # Network setup
  stellar_check_dependencies
  # evm_start
  stellar_setup_keypair

  # Build and deploy Stellar contract factory
  stellar_deploy_escrow_factory

  # Run stages
  deployEscrowDstStellar
  # multichain_run_stages
  
  # Cleanup
  # evm_cleanup
  
  echo "=== All stages completed ==="
}

# Run main function
main "$@"