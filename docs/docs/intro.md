# Solana Security & Performance Framework

This framework serves as a dual-purpose reference implementation for Solana developers.

## 1. Educational Vulnerability Archive
A catalog of 7 critical Solana vulnerability patterns, ranging from basic checks to complex system-level exploits.

## 2. Performance Research Lab
A benchmarking environment comparing the Compute Unit (CU) and memory overhead of "Safe Anchor" abstractions versus "Safe Pinocchio" (raw systems) implementations.

---

## Vulnerability Matrix

| ID | Vulnerability | Category | Tech Focus |
|---|---|---|---|
| 01 | Missing Signer | Sanity | Anchor |
| 02 | Integer Overflow | Core | Rust |
| 03 | PDA Metadata Leak | Privacy | Systems |
| 04 | Zombie Resurrection | Memory | Pinocchio |
| 05 | Type Confusion | Safety | Pinocchio |
| 06 | Unaligned Pointer | Hardware | Pinocchio |
| 07 | Transfer Hook Reentrancy | Complex | Token-22 |
