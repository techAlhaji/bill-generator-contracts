# Security Policy

## 1. Supported Versions

Security updates and patches are actively maintained for the following versions of the StellarBills smart contracts:

| Version | Target Protocol | Status |
| :--- | :--- | :--- |
| `0.1.x` | Protocol 22 (Soroban SDK v22.0.0) | Supported (Active Testnet) |
| `< 0.1.0` | Pre-release | Unsupported |

---

## 2. Unaudited & Testnet Disclaimer

> [!WARNING]
> **Testnet Deployment Notice:** The contracts in this repository are currently deployed on the **Stellar Testnet** for testing, benchmarking, and community integration purposes. They have not yet undergone a formal, independent third-party smart contract security audit. Do **not** deploy these contracts to the Stellar Mainnet or use them with real monetary funds without completing a comprehensive security audit and formal verification.

---

## 3. Reporting a Vulnerability

We take the security of our smart contracts, cryptographic commitments, and user funds seriously. If you identify a security vulnerability, please disclose it responsibly.

### Responsible Disclosure Process
- **Contact:** Email the maintainer directly at [`ademolabuild@gmail.com`](mailto:ademolabuild@gmail.com).
- **Subject:** `[SECURITY] StellarBills Contract Vulnerability Report`
- **Information to Include:**
  - Contract function and file name affected.
  - Detailed description of the potential exploit scenario or attack vector.
  - Step-by-step reproduction instructions or a minimal test case replicating the bug.
  - Potential economic or ledger state impact.

### Response Timeline
- **Initial Response:** Within 48 hours acknowledging receipt of your report.
- **Triage & Assessment:** Within 5 business days outlining confirmed severity and remediation steps.
- **Patch & Public Disclosure:** A coordinated public disclosure will take place once a fix has been deployed and verified.

Please do **not** open public GitHub issues or discuss security vulnerabilities publicly before the vulnerability has been triaged and resolved.
