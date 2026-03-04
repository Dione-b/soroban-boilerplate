#!/bin/bash
# Script to deploy the specified contract
set -e

if [ -z "$1" ]; then
  echo "Usage: $0 <contract_name>"
  echo "Example: $0 hello_world"
  exit 1
fi

CONTRACT_NAME=$1
shift

wasm_path="target/wasm32v1-none/release/${CONTRACT_NAME}.wasm"

if [ ! -f "$wasm_path" ]; then
  wasm_path="target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.wasm"
fi

if [ ! -f "$wasm_path" ]; then
  echo "Error: File $wasm_path not found! Compile the contract first with 'stellar contract build'"
  exit 1
fi

echo "Deploying $CONTRACT_NAME to testnet..."

stellar contract deploy \
  --wasm "$wasm_path" \
  --source alice \
  --network testnet \
  "$@"
