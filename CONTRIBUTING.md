# Contributing to StellarBills Contracts

Thank you for your interest in contributing to the **StellarBills** Soroban smart contracts. This repository contains the on-chain invoice commitment registry and settlement coordinator for the StellarBills protocol.

To ensure high code quality, security, and consistent git history, all contributors must follow these guidelines.

---

## 1. Claiming an Issue

We use an issue-first workflow:
1. Browse open issues on the [Issues Tab](https://github.com/techAlhaji/bill-generator-contracts/issues).
2. Comment `.take` on the issue you wish to work on.
3. Wait for a maintainer to assign the issue to you before writing code.
4. Only work on one issue at a time.

### Complexity Tiers

| Label | Scope | Estimated Effort |
| :--- | :--- | :--- |
| `complexity:low` | Documentation updates, test coverage additions, minor helper methods | < 2 hours |
| `complexity:medium` | Storage optimization, new event emission, error handling refinement | 2–5 hours |
| `complexity:high` | New contract logic, cross-contract calls, security and auth re-architecting | > 5 hours |

---

## 2. Development & Toolchain Setup

### Prerequisites
- **Rust:** `1.84+`
- **Target:** `wasm32-unknown-unknown`
- **Components:** `clippy`, `rustfmt`
- **Soroban SDK:** `v22.0.0` (Protocol 22)

```bash
# Add WebAssembly compilation target
rustup target add wasm32-unknown-unknown

# Add linters
rustup component add clippy rustfmt
```

### Local Verification Commands
All changes must pass before opening a pull request:

```bash
# 1. Run unit and integration tests
cargo test --all

# 2. Check code formatting
cargo fmt --all -- --check

# 3. Run Clippy with zero warnings tolerated
cargo clippy --all-targets -- -D warnings

# 4. Compile WebAssembly binary
cargo build --target wasm32-unknown-unknown --release
```

---

## 3. Coding Standards & Soroban Constraints

- **Zero Floating-Point Arithmetic:** Never use `f32` or `f64`. Use `i128` minor units (e.g., stroops for native XLM) and basis points for percentages (`100 bps = 1%`).
- **No Unchecked `unwrap()`:** Production contract logic must return explicit `Result<T, ContractError>`. `unwrap()` is strictly restricted to unit tests.
- **Explicit Authorization:** Every mutating operation must verify authorization with `address.require_auth()`.
- **Storage Lifecycle:** Ensure persistent storage entries have their TTL extended using `env.storage().persistent().extend_ttl(...)`.
- **Event Emission:** Every state change (`register_invoice`, `pay_invoice`, `cancel_invoice`) must emit a typed Soroban event.

---

## 4. Git & Commit Guidelines

- **Branch Naming:**
  - `feat/issue-<number>-short-description`
  - `fix/issue-<number>-short-description`
  - `docs/issue-<number>-short-description`
- **Conventional Commits:** All commit messages must follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:
  - `feat(contracts): add query method for issuer invoice history`
  - `fix(contracts): enforce minimum invoice amount check`
  - `test(contracts): add unit test for cancelled invoice repayment attempt`
- **Atomic Commits:** One logical change per commit. Never batch unrelated files or run unguided `git add .`.

---

## 5. Pull Request Process

1. Push your branch to GitHub.
2. Open a Pull Request against the `main` branch.
3. Link the issue in the description: `Closes #<issue_number>`.
4. Ensure all CI checks (the `Test & Lint` GitHub Actions job) pass.
5. Address any maintainer review feedback promptly.
