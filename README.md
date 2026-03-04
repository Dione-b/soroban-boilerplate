# Soroban Boilerplate

This project is a complete boilerplate for developing smart contracts on Stellar using Soroban.
It was designed to be a reusable skeleton, including example contracts, local tests, and scripts to simplify deployment.

## Project Structure

- `contracts/`: Example contracts demonstrating fundamental Soroban features.
  - `hello_world/`: A simple string-passing contract.
  - `increment/`: A contract with local storage.
  - `token/`: A simplified demo version of a customizable token contract.
- `scripts/`: Automated utilities to facilitate deployment and interaction with contracts.
- `docker-compose.yml`: File to start the isolated local Stellar/Soroban infrastructure using Quickstart.
- `.cursor/`: Cursor-focused documentation and context guides.
- `.env.example`: Example environment variables configuration with links to Stellar networks.

## Prerequisites

1. [Rust](https://rustup.rs/) installed on your host system.
2. Add the `wasm32` target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
3. [Stellar CLI](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup) installed:
   ```bash
   cargo install --locked stellar-cli --features opt
   ```
4. [Docker](https://docs.docker.com/get-docker/) installed, if you want to run the local testnet using Quickstart.

## Stellar CLI Setup

### 1. Create Identities (Keys)

Create a key in the CLI and fund the account via friendbot:

```bash
stellar keys generate alice --network testnet
```

### 2. Local Network using Docker

For a purely isolated environment, start the local quickstart:

```bash
docker-compose up
```

## How to use (Workflow)

### 1. Compiling the Contracts

It is recommended to use the native Stellar CLI utility to compile optimally for `wasm` with reduced size.

```bash
stellar contract build
```

### 2. Running Unit Tests

The boilerplate already has automated tests. To ensure safety:

```bash
cargo test
```

Or use the provided test script:

```bash
./scripts/test_local.sh
```

### 3. Deploy Contracts (to testnet)

Use the provided automated deploy script (which will use the `alice` key):

```bash
./scripts/deploy.sh hello_world
./scripts/deploy.sh increment
./scripts/deploy.sh token
```

### 4. Interact with a published contract

After deployment, keep the Contract ID that appears in your console and invoke functions:

```bash
./scripts/interact.sh <CONTRACT_ID> hello --to Soroban
```

## Customization

Add new contract directories to the root of `contracts/` and, if necessary, update the workspace `Cargo.toml` file. It's always recommended to write the respective test functions in the `/tests` folder or inline modules to guarantee test coverage.
