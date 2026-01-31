# Solana Security & Performance Framework

Welcome to **BytesUnchecked**, a specialized framework for Solana developers to explore critical security patterns and performance trade-offs.

## The Philosophy: Performance Security
Modern blockchain development often forces a choice between the high-level safety of frameworks like **Anchor** and the raw performance of low-level **Pinocchio** (raw Rust) implementations. 

This project demonstrates that performance and security are not mutually exclusive. By enforcing rigorous manual invariants—such as explicit discriminators, alignment checks, and the Checks-Effects-Interactions (CEI) pattern—developers can achieve "Pinocchio performance" with "Anchor-grade safety."

## What's Inside?

### 1. Educational Vulnerability Archive
A side-by-side comparison of **Insecure** vs. **Secure** implementations for 7 common Solana vulnerability patterns. Each example includes:
- **The Exploit**: A TypeScript test demonstrating how the vulnerability can be abused.
- **The Fix**: A secure implementation using industry standards.
- **The Analysis**: A deep dive into the underlying memory and logic mechanics.

### 2. Performance Research Lab
We utilize `mollusk-svm` to measure the exact Compute Unit (CU) cost of every security check. Our goal is to provide a benchmark for:
- Anchor's abstraction overhead.
- The cost of manual safety rails in raw implementations.

---

## The Vulnerability Matrix

| ID | Vulnerability | Category | Core Tech | Focus |
|---|---|---|---|---|
| 01 | [Missing Signer](./vulnerabilities/01-missing-signer) | Sanity | Anchor | Identity Validation |
| 02 | [Arithmetic Overflow](./vulnerabilities/02-arithmetic-overflow) | Logic | Rust | Checked Math |
| 03 | [PDA Metadata Leak](./vulnerabilities/03-pda-seed-leak) | Privacy | Systems | Salted Hashes |
| 04 | [Zombie Resurrection](./vulnerabilities/04-account-closing) | Memory | Pinocchio | Data Zeroing |
| 05 | [Type Confusion](./vulnerabilities/05-type-confusion) | Safety | Pinocchio | Account Discriminators |
| 06 | [Unaligned Pointer](./vulnerabilities/06-zero-copy-alignment) | Hardware | Pinocchio | Zero-Copy Safety |
| 07 | [Hook Reentrancy](./vulnerabilities/07-hook-reentrancy) | Complex | Token-22 | CEI Pattern |