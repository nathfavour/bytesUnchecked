# BytesUnchecked

Welcome to **BytesUnchecked**, a reference repository for Solana developers to explore critical security patterns and their implementations in Anchor.

## Overview

This project provides a side-by-side comparison of **Insecure** vs. **Secure** implementations for 7 common Solana vulnerability patterns.

### 1. Vulnerability Examples
Each example includes:
- **The Exploit**: A TypeScript test demonstrating how the vulnerability can be abused.
- **The Fix**: A secure implementation using Anchor framework best practices.
- **The Analysis**: A breakdown of the security flaw and the mitigation strategy.

---

## The Vulnerability Matrix

| ID | Vulnerability | Category | Focus |
|---|---|---|---|
| 01 | [Missing Signer](./vulnerabilities/01-missing-signer) | Signature Verification | Identity Validation |
| 02 | [Arithmetic Overflow](./vulnerabilities/02-arithmetic-overflow) | Logic | Checked Math |
| 03 | [PDA Seed Leak](./vulnerabilities/03-pda-seed-leak) | Privacy | PDA Security |
| 04 | [Account Closing](./vulnerabilities/04-account-closing) | Memory | Data Zeroing |
| 05 | [Type Confusion](./vulnerabilities/05-type-confusion) | Account Validation | Discriminators |
| 06 | [Zero-Copy Alignment](./vulnerabilities/06-zero-copy-alignment) | Memory | Memory Layout |
| 07 | [Hook Reentrancy](./vulnerabilities/07-hook-reentrancy) | CPI Logic | CEI Pattern |
