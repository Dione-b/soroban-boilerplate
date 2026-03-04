#!/bin/bash
# Script to interact with a contract on the testnet
set -e

if [ "$#" -lt 2 ]; then
  echo "Usage: $0 <contract_id> <function_name> [arguments...]"
  echo "Example: $0 CD...XV hello --to Maria"
  exit 1
fi

CONTRACT_ID=$1
FUNCTION_NAME=$2
shift 2

echo "Invoking function $FUNCTION_NAME on contract $CONTRACT_ID..."

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source alice \
  --network testnet \
  -- \
  "$FUNCTION_NAME" \
  "$@"
