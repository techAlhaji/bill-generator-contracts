# bill-generator-contracts

Soroban smart contracts for the Stellar Bill Generator platform.

This repository contains the `invoice_registry` smart contract built with Rust using official Soroban SDK `v22.0.0` (Protocol 22). It serves as the on-chain proof-of-issuance anchor, privacy-preserving commitment registry, and atomic settlement coordinator.

---

## 1. Specifications & Architecture

- **Detailed Specification:** [`docs/CONTRACT_SPEC.md`](./docs/CONTRACT_SPEC.md)
- **Root Architecture Plan:** [`../PROJECT_PLAN.md`](../PROJECT_PLAN.md)
- **Backend API Contract:** [`../bill-generator-backend/openapi.yaml`](../bill-generator-backend/openapi.yaml)
- **Frontend Integration Guide:** [`../bill-generator-frontend/docs/INTEGRATION_SPEC.md`](../bill-generator-frontend/docs/INTEGRATION_SPEC.md)

---

## 2. Requirements & Installation

### Toolchain Prerequisites
- **Rust:** `1.84+` (managed via `rustup`)
- **Compilation Target:** `wasm32-unknown-unknown`
- **Stellar CLI:** `stellar-cli` `v22+` (optional, for testnet deployment and client binding generation)

### Installation (WSL Ubuntu / Linux)
```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# 2. Add Wasm target
rustup target add wasm32-unknown-unknown

# 3. Add components
rustup component add clippy rustfmt
```

---

## 3. Build & Test Commands

```bash
# Run unit tests
cargo test

# Build release WebAssembly binary
cargo build --target wasm32-unknown-unknown --release

# Format check
cargo fmt --all -- --check

# Linter check
cargo clippy --all-targets -- -D warnings
```

Or using the included `Makefile`:
```bash
make test
make build
make clippy
make fmt
```

---

## 4. Contract Methods & Interface

| Method | Parameters | Authorization | Description |
| :--- | :--- | :--- | :--- |
| `initialize` | `admin: Address` | None (Callable once) | Sets contract admin and emits `init` event. |
| `register_invoice` | `issuer: Address`, `invoice_id: BytesN<32>`, `amount: i128`, `token: Address`, `content_hash: BytesN<32>` | `issuer.require_auth()` | Commits invoice on-chain with `Unpaid` status. Prevents duplicates. |
| `pay_invoice` | `payer: Address`, `invoice_id: BytesN<32>` | `payer.require_auth()` | Transfers tokens from payer to issuer via SEP-41 token client, marks status `Paid`. |
| `record_direct_settlement` | `issuer: Address`, `invoice_id: BytesN<32>`, `payment_ref: BytesN<32>` | `issuer.require_auth()` | Reconciles verified direct L1 Stellar Classic payment on-chain without double-charging tokens. |
| `cancel_invoice` | `issuer: Address`, `invoice_id: BytesN<32>` | `issuer.require_auth()` | Transitions unpaid invoice to `Cancelled`. Prohibited if already paid. |
| `get_invoice` | `invoice_id: BytesN<32>` | Read-only | Returns `Option<InvoiceRecord>` and extends persistent storage TTL. |
| `get_admin` | None | Read-only | Returns `Option<Address>` of the contract admin. |

---

## 5. Testnet Deployment Guide

```bash
# 1. Configure Stellar Testnet
stellar network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# 2. Generate and fund deployer identity
stellar keys generate --global deployer --network testnet
stellar keys fund deployer --network testnet

# 3. Deploy contract Wasm
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/invoice_registry.wasm \
  --source deployer \
  --network testnet
# Outputs deployed CONTRACT_ID (e.g. CA...)

# 4. Initialize contract
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --admin $(stellar keys address deployer)

# 5. Generate TypeScript Client Bindings
stellar contract bindings typescript \
  --wasm target/wasm32-unknown-unknown/release/invoice_registry.wasm \
  --id <CONTRACT_ID> \
  --output-dir ../packages/invoice-registry-client
```

---

## 6. Limitations of the MVP Payment Model

1. **Direct L1 Payment Reconciliation:** Direct payments made via Stellar Classic operations (e.g. XLM or USDC transfers with invoice memo) execute on the Stellar Core L1 ledger. Because Soroban contracts cannot autonomously listen to L1 transactions without an oracle or off-chain relayer, the merchant/issuer (or an authorized relayer) must invoke `record_direct_settlement` with the verified transaction hash to update the on-chain status.
2. **Strict Single-Transaction Settlement:** To prevent undercollateralized releases, the contract requires full payment in a single token transfer. Partial payments are rejected at the contract level.
3. **Storage TTL Expiration:** Persistent invoice entries have a 30-day TTL extension window (`LEDGER_EXTEND_TO = 518400`). To retain data indefinitely on testnet/mainnet, entries must be read or refreshed before the expiration threshold (`LEDGER_THRESHOLD = 17280`).