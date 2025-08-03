# ETH Unite DeFi 2025

## Create a new contract interface:

```bash
stellar contract init . --name adder_interface
stellar contract init . --name add_extra_contract
```

## Build

```bash
cd cross-chain-swap
stellar contract build

cargo test
```

## Deploy

```
./deploy_full_stack.sh
```

## Generate binindgs

Build first


# Client-side call

- make sure you executed `./deploy_full_stack.sh` to start the network and get contract addresses
- edit configuration folder in client/config/config.json

```bash
cd client
bun install
bun index.ts
```