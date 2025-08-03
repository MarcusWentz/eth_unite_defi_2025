#!/bin/bash
set -e

# Deploy contracts and capture addresses
echo "Deploying contracts..."

# Deploy base escrow
BASE_ESCROW_ADDRESS=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/base_escrow.wasm --source admin --network testnet --rpc-url http://localhost:8000/soroban/rpc/v1)
echo "BASE_ESCROW_ADDRESS=$BASE_ESCROW_ADDRESS"

# Deploy escrow factory
ESCROW_FACTORY_ADDRESS=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/escrow.wasm --source admin --network testnet --rpc-url http://localhost:8000/soroban/rpc/v1)
echo "ESCROW_FACTORY_ADDRESS=$ESCROW_FACTORY_ADDRESS"

# Deploy order protocol
ORDER_PROTOCOL_ADDRESS=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/order.wasm --source admin --network testnet --rpc-url http://localhost:8000/soroban/rpc/v1)
echo "ORDER_PROTOCOL_ADDRESS=$ORDER_PROTOCOL_ADDRESS"

# Deploy resolver
RESOLVER_ADDRESS=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/resolver.wasm --source admin --network testnet --rpc-url http://localhost:8000/soroban/rpc/v1)
echo "RESOLVER_ADDRESS=$RESOLVER_ADDRESS"

# Deploy test token
TEST_TOKEN_ADDRESS=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/test_token.wasm --source admin --network testnet --rpc-url http://localhost:8000/soroban/rpc/v1)
echo "TEST_TOKEN_ADDRESS=$TEST_TOKEN_ADDRESS"

# Save addresses to .env file
cat > contracts/.env << EOL
BASE_ESCROW_ADDRESS=$BASE_ESCROW_ADDRESS
ESCROW_FACTORY_ADDRESS=$ESCROW_FACTORY_ADDRESS
ORDER_PROTOCOL_ADDRESS=$ORDER_PROTOCOL_ADDRESS
RESOLVER_ADDRESS=$RESOLVER_ADDRESS
TEST_TOKEN_ADDRESS=$TEST_TOKEN_ADDRESS
EOL

echo "Deployment completed successfully!"
