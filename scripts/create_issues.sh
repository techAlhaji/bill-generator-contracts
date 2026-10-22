#!/usr/bin/env bash
# scripts/create_issues.sh
# Automates bulk issue creation for techAlhaji/bill-generator-contracts using GitHub CLI (gh)

set -e

REPO="techAlhaji/bill-generator-contracts"

echo "Creating labels on $REPO..."
gh label create "complexity:low" --repo "$REPO" --color "0E8A16" --description "Good for newcomers, < 2 hours" --force || true
gh label create "complexity:medium" --repo "$REPO" --color "FBCA04" --description "Moderate complexity, 2-5 hours" --force || true
gh label create "complexity:high" --repo "$REPO" --color "D93F0B" --description "High complexity architectural task, > 5 hours" --force || true
gh label create "type:feat" --repo "$REPO" --color "1D76DB" --description "New feature or capability" --force || true
gh label create "type:test" --repo "$REPO" --color "C5DEF5" --description "Test coverage expansion" --force || true
gh label create "type:docs" --repo "$REPO" --color "5319E7" --description "Documentation and guides" --force || true
gh label create "type:refactor" --repo "$REPO" --color "F9D0C4" --description "Code refactoring and optimization" --force || true
gh label create "type:ci" --repo "$REPO" --color "BFD4F2" --description "CI/CD and automation" --force || true

echo "Creating planned issues on $REPO..."

# Issue 1
gh issue create --repo "$REPO" \
  --title "feat(contracts): implement batch invoice commitment registration" \
  --label "complexity:medium,type:feat" \
  --body "### Summary
Enable merchants to commit multiple invoices in a single transaction invocation via \`register_invoices_batch(issuer: Address, invoices: Vec<InvoiceCommitmentPayload>)\`. This amortizes base transaction fees and ledger state overhead during high-volume commercial billing cycles.

### Acceptance Criteria
- [ ] Implement \`InvoiceCommitmentPayload\` struct containing \`invoice_id\`, \`amount\`, \`token\`, and \`content_hash\`.
- [ ] Add \`register_invoices_batch\` to \`InvoiceRegistry\` with atomic commit (all succeed or none).
- [ ] Enforce \`issuer.require_auth()\` once for the whole batch.
- [ ] Add unit test with 10 invoices in a single call verifying state commitments.
- [ ] Document gas/stroop efficiency compared to sequential calls.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0"

# Issue 2
gh issue create --repo "$REPO" \
  --title "feat(contracts): add query method for issuer active invoice count" \
  --label "complexity:low,type:feat" \
  --body "### Summary
Add a read-only instance query method \`get_issuer_stats(issuer: Address) -> IssuerStats\` that returns the cumulative count of registered, paid, and cancelled invoices for an issuer.

### Acceptance Criteria
- [ ] Define \`IssuerStats\` struct (\`total_registered: u64\`, \`total_paid: u64\`, \`total_cancelled: u64\`).
- [ ] Update counters atomically during \`register_invoice\`, \`pay_invoice\`, and \`cancel_invoice\`.
- [ ] Ensure TTL on stats record is extended on access.
- [ ] Add unit tests verifying counter increments.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0"

# Issue 3
gh issue create --repo "$REPO" \
  --title "test(contracts): add comprehensive test cases for state TTL extension under heavy simulation" \
  --label "complexity:medium,type:test" \
  --body "### Summary
Expand integration tests to verify entry lifetime management (\`extend_ttl\`) for persistent storage records across simulated ledger jumps up to Protocol 22 maximum threshold.

### Acceptance Criteria
- [ ] Write integration test verifying \`InvoiceRecord\` survives 50,000 ledger advancement.
- [ ] Verify that accessing expired record triggers appropriate error response.
- [ ] Verify \`get_invoice\` properly extends instance and persistent storage TTL.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0 test harness"

# Issue 4
gh issue create --repo "$REPO" \
  --title "feat(contracts): emit standardized SEP-41 payment event on direct settlement reconciliation" \
  --label "complexity:low,type:feat" \
  --body "### Summary
When an invoice is reconciled via \`record_direct_settlement\`, ensure a specialized event \`direct_settlement_recorded\` is emitted with topics \`[Symbol::new(&env, \"direct_pay\"), issuer, invoice_id]\` and data \`payment_ref\` to facilitate indexer ingestion.

### Acceptance Criteria
- [ ] Define distinct topic symbol for direct L1 settlements.
- [ ] Verify event payload matches schema expected by off-chain indexers.
- [ ] Add test asserting event emission using \`env.events().all()\`.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0"

# Issue 5
gh issue create --repo "$REPO" \
  --title "refactor(contracts): optimize storage footprint with custom DataKey enum packing" \
  --label "complexity:medium,type:refactor" \
  --body "### Summary
Audit the \`DataKey\` enum variants and struct representations to minimize serialized XDR byte size in ledger persistent storage, reducing rent costs for merchants.

### Acceptance Criteria
- [ ] Audit \`InvoiceRecord\` serialization footprint.
- [ ] Ensure all enum variants use minimal discriminant representations.
- [ ] Run benchmark measuring rent cost per registered invoice before and after optimization.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0"

# Issue 6
gh issue create --repo "$REPO" \
  --title "feat(contracts): add contract upgradeability governance mechanism via admin auth" \
  --label "complexity:high,type:feat" \
  --body "### Summary
Implement a secure, multi-step contract upgrade mechanism (\`upgrade(new_wasm_hash: BytesN<32>)\`) utilizing \`env.deployer().update_current_contract_wasm()\` restricted strictly to \`admin.require_auth()\`.

### Acceptance Criteria
- [ ] Implement \`upgrade\` method restricted to contract admin.
- [ ] Add 24-hour timelock or multi-sig preparation step for upgrade safety.
- [ ] Write integration test upgrading contract WASM and asserting migrated storage integrity.
- [ ] Document upgrade procedures and emergency safeguards in \`docs/UPGRADES.md\`.

### Tech Stack
- Rust 1.84+
- soroban-sdk v22.0.0"

# Issue 7
gh issue create --repo "$REPO" \
  --title "docs(contracts): add formal state transition diagrams to docs/CONTRACT_SPEC.md" \
  --label "complexity:low,type:docs" \
  --body "### Summary
Enhance \`docs/CONTRACT_SPEC.md\` with Mermaid state transition diagrams illustrating invoice status changes (\`Unpaid\` -> \`Paid\`, \`Unpaid\` -> \`Cancelled\`), authorization rules, and storage TTL policies.

### Acceptance Criteria
- [ ] Add Mermaid state machine diagram.
- [ ] Add sequence diagram for \`pay_invoice\` cross-contract transfer.
- [ ] Add sequence diagram for \`record_direct_settlement\` reconciliation.

### Tech Stack
- Markdown, Mermaid.js"

# Issue 8
gh issue create --repo "$REPO" \
  --title "ci(contracts): add cargo-audit and cargo-deny security scanning to GitHub Actions workflow" \
  --label "complexity:low,type:ci" \
  --body "### Summary
Integrate automated vulnerability scanning in \`.github/workflows/ci.yml\` using \`cargo-audit\` for known Rust crate advisory databases and \`cargo-deny\` for license and duplicate dependency compliance.

### Acceptance Criteria
- [ ] Add \`cargo-audit\` step to CI pipeline.
- [ ] Add \`cargo-deny\` configuration file.
- [ ] Ensure CI fails on active security advisories.

### Tech Stack
- GitHub Actions, Cargo"

echo "All planned issues successfully created for $REPO!"
