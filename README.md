# Soroban Boilerplate

This project is a boilerplate for developing smart contracts on Stellar using Soroban. It automatically includes example contracts, automation scripts, integration tests, and reference documentation.

## Project Structure

- `.cursor/`: Comprehensive documentation of the Stellar ecosystem (used by Cursor editor).
- `contracts/`: Example contracts.
  - `hello_world`: A simple contract that returns a greeting.
  - `increment`: A contract with a persistent counter (demonstrates storage and TTL usage).
  - `token`: Basic token implementation (fungible).
- `scripts/`: Utilities for deploying, interaction, and local testing.
- `tests/`: Integration tests encompassing multiple contracts.
- `docker-compose.yml`: Spins up a local Stellar network for testing.
- `.env.example`: Environment configuration template example.

## Prerequisites

- Rust (with `wasm32-unknown-unknown` target installed)
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)
- Docker (optional, for local network)
- Node.js (optional, if you're using an example frontend – not included by default)

## Initial Setup

1. Clone this repository.
2. Copy `.env.example` to `.env` and adjust the variables based on your desired test network (testnet is standard).
3. If using testnet, generate an identity and fund it via Friendbot:
   ```bash
   stellar keys generate alice --network testnet --fund
   ```
4. For the local network, start the container:
   ```bash
   docker-compose up -d
   stellar keys generate alice --network local --fund
   ```

## Compiling Contracts

To compile all contracts:

```bash
stellar contract build
# or
cargo build --target wasm32-unknown-unknown --release
```

The resulting `.wasm` files will be generated in `target/wasm32-unknown-unknown/release/`.

## Tests

- **Unit Tests** (Inside each contract):
  ```bash
  cargo test
  ```
- **Integration Tests** (Between multiple contracts):
  ```bash
  cargo test --test integration
  ```
- **Local Network Tests** (Using Quickstart):
  ```bash
  ./scripts/test_local.sh
  ```

## Deploy

To deploy a contract on your configured network:

```bash
./scripts/deploy.sh <contract_name>
```

Example:

```bash
./scripts/deploy.sh hello_world
```

The script defaults to the `alice` identity. You can easily alter the script to accept other identity inputs.

## Interacting with Contracts

Use the `interact.sh` script to invoke functions seamlessly:

```bash
./scripts/interact.sh <contract_id> <function> [arguments...]
```

Example (invoking `hello` from the hello_world contract):

```bash
./scripts/interact.sh C... hello --to World
```

For more complex commands, consult the [Stellar CLI documentation](https://developers.stellar.org/docs/tools/stellar-cli).

## Reference Documentation

The `.cursor` folder contains detailed guides about multiple topics:

- `contracts-soroban.md`: Smart contract development guide.
- `frontend-stellar-sdk.md`: Integrating with frontends safely.
- `security.md`: Comprehensive security checklist.
- `testing.md`: Validated testing methodologies and strategies.
- `stellar-assets.md`: Instructions for emitting assets.
- `api-rpc-horizon.md`: Data accessing and Horizon usage.
- `advanced-patterns.md`: Advance smart contract design patterns.
- `common-pitfalls.md`: Famous mistakes and their solutions.
- `zk-proofs.md`: Zero Knowledge Proofs usage.
- `ecosystem.md`: Great mapping of ecosystem tools/projects.
- `resources.md`: Useful links and tooling.
- `standards-reference.md`: A quick cheat sheet of SEPs/CAPs.
- `SKILL.md`: Main orchestrator file that feeds context to AI.

These context rules were inspired by and acquired from the [stellar-dev-skill](https://github.com/stellar/stellar-dev-skill) repository. They're heavily optimized if you use an AI-assisted editor like Cursor.

## Customization

To add a brand new contract to the workspace:

1. Build a new folder inside `contracts/` using your target name.
2. Inside it, place a `Cargo.toml` carrying:

   ```toml
   [package]
   name = "my-contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   soroban-sdk = { workspace = true }

   [dev-dependencies]
   soroban-sdk = { workspace = true, features = ["testutils"] }
   ```

3. Type out your code straight onto `src/lib.rs` alongside `src/test.rs`.
4. Include the project into your root workspace `Cargo.toml` if it's placed somewhere other than `contracts/*` directory scope.

## Contribution

Issues and pull requests presenting better implementations are more than welcome.
