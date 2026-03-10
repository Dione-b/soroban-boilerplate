# 🚀 Soroban Boilerplate (AI-Assisted Development)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.84%2B-blue)](https://www.rust-lang.org)
[![Stellar](https://img.shields.io/badge/Stellar-Soroban-7B42F5)](https://stellar.org)

A complete and opinionated template for developing smart contracts on **Stellar** using **Soroban**.  
This boilerplate was created to **accelerate the start of new projects** and is specially crafted for **vibe coding** (AI-assisted development), providing:

- 📦 **Rust workspace structure** with multiple example contracts.
- 🔧 **Ready-to-use scripts** for deployment, interaction, and testing.
- 🧪 **Unit and integration tests** configured.
- 🤖 **Antigravity & Cursor Ready** – Comes with pre-configured rules and skills in `.agents/` and `.cursor/` to make your AI assistant a Soroban expert.
- 📚 **Extensive documentation** – Acting as the brain for your AI assistant so you can focus on the vibe.
- 🐳 **Local network support** via Docker (Stellar Quickstart).
- 🔐 **Security best practices** and advanced patterns.

Whether you are starting with Soroban or want a solid starting point for a real project using the latest AI-driven workflows (like **Antigravity**), this boilerplate is for you.

---

## 📁 Project Structure

```
.
├── .agents/                     # Antigravity Agent Configuration (Skills & Workflows)
│   ├── skills/stellar-dev/      # Soroban specialized knowledge
│   └── workflows/project-rules.md # Project-specific AI guidelines
├── .cursor/                     # Complete documentation of the Stellar ecosystem
│   └── ... (13 documentation files)
├── .cursorrules                 # Legacy rules for the Cursor editor
├── .env.example                 # Example environment variables
├── Cargo.toml                   # Main workspace
├── docker-compose.yml           # Local Stellar network
├── README.md                    # You are here!
├── contracts/                   # Example contracts
│   ├── hello_world/             # Simple contract (greeting)
│   ├── increment/               # Persistent counter (storage and TTL)
│   └── token/                   # Basic fungible token implementation
├── scripts/                     # Utilities
│   ├── deploy.sh
│   ├── interact.sh
│   └── test_local.sh
└── tests/                       # Integration tests
    └── integration.rs
```

---

## 🛠️ Prerequisites

Before you start, make sure you have installed:

- **Rust** (version 1.84 or higher) – [official installation](https://www.rust-lang.org/tools/install)
  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- **Stellar CLI** – [installation guide](https://developers.stellar.org/docs/tools/stellar-cli)
  ```bash
  cargo install stellar-cli --locked
  ```
- **Docker** (optional, for local network) – [install Docker](https://docs.docker.com/get-docker/)
- **Node.js** (optional, if you plan to add a frontend later)

---

## ⚙️ Initial Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/Dione-b/soroban-boilerplate.git
   cd soroban-boilerplate
   ```

2. **Configure environment variables**

   ```bash
   cp .env.example .env
   # Edit .env with your desired network (testnet is the default)
   ```

3. **Create and fund an identity (for testnet)**
   ```bash
   stellar keys generate alice --network testnet --fund
   ```
   For local network:
   ```bash
   docker-compose up -d
   stellar keys generate alice --network local --fund
   ```

---

## 🔨 Compiling Contracts

To compile all contracts at once:

```bash
stellar contract build
# or
cargo build --target wasm32-unknown-unknown --release
```

The `.wasm` files will be generated in `target/wasm32-unknown-unknown/release/`.

**Expected output example:**

```
$ stellar contract build
   Compiling hello_world v0.0.0
   Compiling increment v0.0.0
   Compiling token v0.0.0
    Finished release [optimized] target(s) in 15.2s
```

---

## 🧪 Tests

### Unit tests (inside each contract)

```bash
cargo test
```

Expected output:

```
running 3 tests
test test::test_hello ... ok
test test::test_increment ... ok
test test::test_token ... ok
```

### Integration tests (cross-contract)

```bash
cargo test --test integration
```

These tests simulate interactions between multiple contracts, validating their compatibility.

### Local network tests (using Quickstart)

```bash
./scripts/test_local.sh
```

This script boots up the local network (via Docker), deploys the contracts, and runs an automated test suite.

> 💡 **Tip**: Tests generate **snapshots** automatically in the `test_snapshots/` folder. Compare them in Git to detect unexpected changes in contract behavior.

---

## 🚀 Deploy

To deploy a contract to the configured network:

```bash
./scripts/deploy.sh <contract_name> [--source <identity>] [--network <network>]
```

The script reads defaults from your `.env` file (`STELLAR_NETWORK`, `STELLAR_IDENTITY`). You can override them via CLI flags:

```bash
# Uses defaults from .env (identity: alice, network: testnet)
./scripts/deploy.sh hello_world

# Override identity and network
./scripts/deploy.sh hello_world --source bob --network local
```

**Expected output:**

```
Deploying hello_world...
  Network:  testnet
  Identity: alice
Contract ID: CABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890
```

---

## 🤝 Interacting with Contracts

Use the `interact.sh` script to invoke contract functions:

```bash
./scripts/interact.sh <contract_id> <function_name> [--source <identity>] [--network <network>] [-- args...]
```

Examples:

```bash
# Uses defaults from .env
./scripts/interact.sh CABCDEF...7890 hello -- --to World

# Override identity and network
./scripts/interact.sh CABCDEF...7890 hello --source bob --network local -- --to World
```

Expected output:

```
["Hello", "World"]
```

For more complex commands, refer to the [Stellar CLI documentation](https://developers.stellar.org/docs/tools/stellar-cli).

---

## 📚 Reference Documentation (for humans and AI)

The **`.cursor/`** folder contains **detailed guides** covering all aspects of development with Stellar/Soroban.  
These documents are especially useful if you use the **Cursor** editor, as the AI can automatically browse them to answer your questions.

| File                      | Content                                                   |
| ------------------------- | --------------------------------------------------------- |
| `contracts-soroban.md`    | Contract development (storage, auth, events, errors)      |
| `frontend-stellar-sdk.md` | Frontend integration (Next.js, React, wallets)            |
| `security.md`             | Security checklist and common vulnerabilities             |
| `testing.md`              | Testing strategies (unit, integration, fuzzing, mutation) |
| `stellar-assets.md`       | Asset issuance and SAC usage                              |
| `api-rpc-horizon.md`      | Data access via RPC and Horizon                           |
| `advanced-patterns.md`    | Advanced patterns (upgrade, factory, governance, DeFi)    |
| `common-pitfalls.md`      | Frequent errors and how to solve them                     |
| `zk-proofs.md`            | Zero-knowledge proofs (status-sensitive)                  |
| `ecosystem.md`            | Mapping of tools, projects, and teams                     |
| `resources.md`            | Useful links (official documentation, SDKs, examples)     |
| `standards-reference.md`  | Quick reference for SEPs and CAPs                         |
| `SKILL.md`                | Main AI orchestrator (defines the assistant's scope)      |

---

## 🤖 AI Agent Configuration (Antigravity)

This project is optimized for the **Antigravity** agent. The configuration is stored in the `.agents/` directory:

- **Skills (`.agents/skills/`)**: Contains the `stellar-dev` skill, a comprehensive knowledge base about Stellar and Soroban. The agent automatically consults these files when you ask about contract development, security, or deployment.
- **Workflows (`.agents/workflows/`)**:
  - `project-rules.md`: Defines the global behavior of the agent (e.g., responding in Portuguese, following DRY/KISS principles).
  - Use `/project-rules` to explicitly invoke the project's guidelines.

The agent uses these rules to ensure high-quality code, proper error handling, and adherence to the Stellar ecosystem's best practices without manual intervention.

---

## 🧩 Adding a New Contract

Want to include your own contract? Follow these steps:

1. **Create a folder** in `contracts/` with the desired name:

   ```bash
   mkdir -p contracts/my_contract/src
   ```

2. **Add the `Cargo.toml`**:

   ```toml
   [package]
   name = "my_contract"
   version = "0.0.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   soroban-sdk = { workspace = true }

   [dev-dependencies]
   soroban-sdk = { workspace = true, features = ["testutils"] }
   ```

3. **Write your contract** in `src/lib.rs`:

   ```rust
   #![no_std]
   use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

   #[contract]
   pub struct MyContract;

   #[contractimpl]
   impl MyContract {
       pub fn hello(env: Env, to: String) -> Vec<String> {
           vec![&env, String::from_str(&env, "Hello, "), to]
       }
   }

   #[cfg(test)]
   mod test;
   ```

4. **Add tests** in `src/test.rs` (view example contracts for inspiration).

5. **Done!** The workspace is already configured to automatically include any folder inside `contracts/`.

---

## 🐛 Common Pitfalls

If something is not working as expected, refer to the **`common-pitfalls.md`** file in the `.cursor/` folder. There you will find solutions for:

- Contract is too large (>64KB)
- Missing `#![no_std]` error
- TTL not extended (archived data)
- Authorization failing in tests
- Sequence errors (tx_bad_seq)
- And much more!

Still having issues? Open an [issue](https://github.com/Dione-b/soroban-boilerplate/issues).

---

## 🤝 Contributing

Contributions are **very welcome**! If you have an idea, found a bug, or want to add a new example contract:

1. Fork the project
2. Create a branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

Please keep the code clean, well-tested, and following the best practices documented in the `.cursor/` folder.

---

## 📄 License

This project is licensed under the MIT License – see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- To the Stellar community and all developers contributing to the ecosystem.
- To the [stellar-dev-skill](https://github.com/stellar/stellar-dev-skill) repository that inspired the AI documentation structure.

---

**Now it's up to you!** Clone the repository, explore the examples, and start building the next big project on Stellar. If you have questions, the documentation is just a click (or an AI prompt) away. 🚀
