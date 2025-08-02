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

## Generate binindgs

Build first

```bash

stellar contract bindings typescript --wasm ./cross-chain-swap/target/wasm32v1-none/release/dutch_auction.wasm --output-dir ./client/bindings/dutch_auction

stellar contract bindings typescript --wasm ./cross-chain-swap/target/wasm32v1-none/release/order.wasm --output-dir ./client/bindings/order

```


# Client-side call

- make sure you executed `./deploy_full_stack.sh` to start the network and get contract addresses

```bash
cd client
bun install
bun index.ts
bun index.ts --orderContractAddress "CDVFAOBR5ZJNRT22XOVC7V3X6BKEYMZW5FB5NUAEDO2Y5WA5MNQHAL5K"
```