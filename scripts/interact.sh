#!/bin/bash
# Script to interact with a deployed contract
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
  echo "Usage: $0 <contract_id> <function_name> [--source <identity>] [--network <network>] [-- args...]"
  echo ""
  echo "Options:"
  echo "  --source   Stellar identity to use (default: \$STELLAR_IDENTITY or 'alice')"
  echo "  --network  Stellar network to use (default: \$STELLAR_NETWORK or 'testnet')"
  echo ""
  echo "Examples:"
  echo "  $0 CD...XV hello -- --to Maria"
  echo "  $0 CD...XV hello --source bob --network local -- --to Maria"
  exit 1
}

if [ "$#" -lt 2 ]; then
  usage
fi

CONTRACT_ID=$1
FUNCTION_NAME=$2
shift 2

FUNCTION_ARGS=()

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
    --)
      shift
      FUNCTION_ARGS=("$@")
      break
      ;;
    *)
      FUNCTION_ARGS+=("$1")
      shift
      ;;
  esac
done

echo "Invoking '${FUNCTION_NAME}' on contract ${CONTRACT_ID}..."
echo "  Network:  ${NETWORK}"
echo "  Identity: ${IDENTITY}"

stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- \
  "$FUNCTION_NAME" \
  "${FUNCTION_ARGS[@]}"
