<p align="center">
  <img src="https://raw.githubusercontent.com/techAlhaji/bill-generator-frontend/main/public/images/banner.png" alt="StellarBills Logo" width="640" onerror="this.style.display='none'" />
</p>

<h1 align="center">StellarBills — Soroban Smart Contracts</h1>

<p align="center">
  <strong>On-chain proof-of-issuance anchor, privacy-preserving commitment registry, and atomic settlement coordinator on Stellar.</strong>
</p>

<p align="center">
  <a href="https://github.com/techAlhaji/bill-generator-contracts/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/techAlhaji/bill-generator-contracts/ci.yml?branch=main&label=CI%20Build" alt="CI Build" />
  </a>
  <img src="https://img.shields.io/badge/Rust-1.84%2B-orange.svg" alt="Rust 1.84+" />
  <img src="https://img.shields.io/badge/Soroban%20SDK-v22.0.0-blueviolet.svg" alt="Soroban SDK v22.0.0" />
  <img src="https://img.shields.io/badge/Protocol-22-blue.svg" alt="Protocol 22" />
  <a href="https://stellar.expert/explorer/testnet/contract/CCG6V5QEJZ2QI7EXIRVQXCHUIFUROXUBMZPTSCRJHOO35CAQ2HZ6IJGC">
    <img src="https://img.shields.io/badge/Stellar%20Testnet-Deployed-success.svg" alt="Stellar Testnet Deployed" />
  </a>
  <a href="./LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-green.svg" alt="License" />
  </a>
</p>

---

## 1. What is StellarBills Contracts?

**StellarBills** is a commercial invoicing and billing infrastructure designed for the Stellar blockchain ecosystem. This repository contains the core `invoice_registry` Soroban smart contract. 

Rather than polluting public ledger state with private customer details, item descriptions, and tax numbers, the contract implements a privacy-preserving commitment architecture:
- **Proof of Issuance:** Merchants anchor an immutable 32-byte SHA-256 digest of the canonical RFC 8785 invoice payload on-chain.
- **Atomic On-Chain Settlement:** Payers settle invoices directly with SEP-41 tokens via `pay_invoice`, atomically transferring payment and marking the invoice paid in one transaction.
- **L1 Reconciled Settlements:** Merchants verify direct Stellar Classic payments via Horizon and record settlement commitments on-chain via `record_direct_settlement` without double-charging tokens.
- **Lifecycle Guarantees:** Strict state transitions prevent duplicate registration, replay attacks, or modifying settled invoices.

---

## 2. Maintainers

| Avatar | Maintainer | Role | Contact |
| :---: | :--- | :--- | :--- |
| <img src="https://github.com/techAlhaji.png" width="48" height="48" style="border-radius:50%;" /> | **Ademola (@techAlhaji)** | Lead Protocol Engineer | [`ademolabuild@gmail.com`](mailto:ademolabuild@gmail.com) / GitHub: [@techAlhaji](https://github.com/techAlhaji) |

---

## 3. Active Testnet Deployment

The `invoice_registry` smart contract is actively deployed and initialized on the official **Stellar Testnet** (Protocol 22):

| Parameter | Value |
| :--- | :--- |
| **Contract Name** | `InvoiceRegistry` |
| **Network** | Stellar Testnet (`Test SDF Network ; September 2015`) |
| **Contract Address** | [`CCG6V5QEJZ2QI7EXIRVQXCHUIFUROXUBMZPTSCRJHOO35CAQ2HZ6IJGC`](https://stellar.expert/explorer/testnet/contract/CCG6V5QEJZ2QI7EXIRVQXCHUIFUROXUBMZPTSCRJHOO35CAQ2HZ6IJGC) |
| **WASM SHA-256 Digest** | `258688c77041da6696b4dab6a3328fb00871659d54df09cc235282b260cdf2a8` |
| **Deployer & Admin** | [`GCF7FGZJ6LVPAWFNANXZH7ANA42G5JII5F2UXXAL3WHQ26ZFXYTE73JM`](https://stellar.expert/explorer/testnet/account/GCF7FGZJ6LVPAWFNANXZH7ANA42G5JII5F2UXXAL3WHQ26ZFXYTE73JM) |
| **Contract Creation Tx** | [`c0ddc3dc...`](https://stellar.expert/explorer/testnet/tx/c0ddc3dc00868055e7765af4c40ccbaa4f951447e8015e246337a3870d269d86) |
| **Initialization Tx** | [`e1e0b095...`](https://stellar.expert/explorer/testnet/tx/e1e0b095ec6471f0f91a1c20a2f7851e6396e389018238d3a0ace977300ec62d) |

---

## 4. Contract Methods & Interface

| Method | Parameters | Auth Required | Description |
| :--- | :--- | :--- | :--- |
| `initialize` | `admin: Address` | None (One-time) | Initializes admin address and emits contract `init` event. |
| `register_invoice` | `issuer: Address`, `invoice_id: BytesN<32>`, `amount: i128`, `token: Address`, `content_hash: BytesN<32>` | `issuer.require_auth()` | Records on-chain invoice commitment in `Unpaid` status. Prevents duplicates. |
| `pay_invoice` | `payer: Address`, `invoice_id: BytesN<32>` | `payer.require_auth()` | Atomically transfers tokens from payer to issuer via SEP-41 token client, marks status `Paid`. |
| `record_direct_settlement` | `issuer: Address`, `invoice_id: BytesN<32>`, `payment_ref: BytesN<32>` | `issuer.require_auth()` | Reconciles verified direct L1 Stellar Classic payment on-chain with payment reference. |
| `cancel_invoice` | `issuer: Address`, `invoice_id: BytesN<32>` | `issuer.require_auth()` | Cancels an unpaid invoice. Prohibited if already settled. |
| `get_invoice` | `invoice_id: BytesN<32>` | Read-only | Returns `Option<InvoiceRecord>` and auto-extends persistent storage TTL. |
| `get_admin` | None | Read-only | Returns `Option<Address>` of the contract administrator. |

---

## 5. Quick-Start & Local Build

### Prerequisites
- [Rust 1.84+](https://rustup.rs/)
- Target: `wasm32-unknown-unknown`
- Optional: `stellar-cli` v22+

```bash
# 1. Clone repository
git clone https://github.com/techAlhaji/bill-generator-contracts.git
cd bill-generator-contracts

# 2. Add Wasm target & linters
rustup target add wasm32-unknown-unknown
rustup component add clippy rustfmt

# 3. Run unit tests
cargo test --all

# 4. Check formatting & Clippy lints
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings

# 5. Build optimized release WebAssembly binary
cargo build --target wasm32-unknown-unknown --release
```

---

## 6. Repository Architecture

```
bill-generator-contracts/
├── .github/
│   └── workflows/
│       └── ci.yml               # Automated formatting, linting, tests & WASM build
├── contracts/
│   └── invoice_registry/
│       ├── src/
│       │   ├── lib.rs           # Core contract implementation & entrypoints
│       │   ├── types.rs         # Storage types, enums (Status, DataKey), structs
│       │   ├── errors.rs        # ContractError enum with typed error codes
│       │   ├── events.rs        # Typed Soroban event topics and payloads
│       │   └── test.rs          # Unit & integration test suite
│       └── Cargo.toml
├── docs/
│   └── CONTRACT_SPEC.md         # Formal technical specification & state machine
├── scripts/
│   └── create_issues.sh         # gh script for automated community issue creation
├── Cargo.toml                   # Cargo workspace definition
├── CONTRIBUTING.md              # Issue claiming workflow (.take) and guidelines
├── SECURITY.md                  # Responsible disclosure policy and disclaimer
└── README.md
```

---

## 7. Contributing & Security

- **Contributing:** Please review [CONTRIBUTING.md](./CONTRIBUTING.md) before submitting code. We use an issue-first workflow with `.take` comments and conventional commits.
- **Security:** Review [SECURITY.md](./SECURITY.md) for vulnerability disclosure guidelines and testnet disclaimer.

---

## 8. Contributors

Thanks to everyone contributing to StellarBills!

<a href="https://github.com/techAlhaji/bill-generator-contracts/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=techAlhaji/bill-generator-contracts" alt="Contributors" />
</a>

---

## 9. License

This repository is licensed under the [Apache License 2.0](./LICENSE).