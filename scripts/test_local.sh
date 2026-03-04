#!/bin/bash
# Script to run tests in a local environment using the containerized testnet
set -e

echo "Starting unit tests..."
cargo test

echo "All tests passed!"
