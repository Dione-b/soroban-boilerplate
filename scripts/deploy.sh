#!/bin/bash
# Script to deploy the specified contract
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENV_FILE="${SCRIPT_DIR}/../.env"

if [ -f "$ENV_FILE" ]; then
  set -a
  source "$ENV_FILE"
  set +a
fi

NETWORK="${STELLAR_NETWORK:-testnet}"
IDENTITY="${STELLAR_IDENTITY:-alice}"

usage() {
  echo "Usage: $0 <contract_name> [--source <identity>] [--network <network>]"
  echo ""
  echo "Options:"
  echo "  --source   Stellar identity to use (default: \$STELLAR_IDENTITY or 'alice')"
  echo "  --network  Stellar network to deploy to (default: \$STELLAR_NETWORK or 'testnet')"
  echo ""
  echo "Examples:"
  echo "  $0 hello_world"
  echo "  $0 hello_world --source bob --network local"
  exit 1
}

if [ -z "$1" ]; then
  usage
fi

CONTRACT_NAME=$1
shift

EXTRA_ARGS=()

while [ "$#" -gt 0 ]; do
  case "$1" in
    --source)
      IDENTITY="$2"
      shift 2
      ;;
    --network)
      NETWORK="$2"
      shift 2
      ;;
    *)
      EXTRA_ARGS+=("$1")
      shift
      ;;
  esac
done

wasm_path="target/wasm32v1-none/release/${CONTRACT_NAME}.wasm"

if [ ! -f "$wasm_path" ]; then
  wasm_path="target/wasm32-unknown-unknown/release/${CONTRACT_NAME}.wasm"
fi

if [ ! -f "$wasm_path" ]; then
  echo "Error: WASM file not found for '${CONTRACT_NAME}'."
  echo "Compile the contract first with: stellar contract build"
  exit 1
fi

echo "Deploying ${CONTRACT_NAME}..."
echo "  Network:  ${NETWORK}"
echo "  Identity: ${IDENTITY}"

stellar contract deploy \
  --wasm "$wasm_path" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  "${EXTRA_ARGS[@]}"
