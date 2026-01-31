# BytesUnchecked: Solana Security & Performance Framework

[![CI](https://github.com/nathfavour/bytesUnchecked/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/nathfavour/bytesUnchecked/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**BytesUnchecked** is a dual-purpose reference implementation designed for Solana developers. It serves as both an educational archive of critical vulnerabilities and a performance research lab for comparing framework overheads.

---

## 🚀 Overview

The core philosophy of this project is **"Performance Security"**: demonstrating that low-level optimization (Pinocchio/Raw Rust) does not require sacrificing safety if rigorous manual invariants (Discriminators, Alignment, CEI Pattern) are enforced.

### 1. Educational Vulnerability Archive
A catalog of 7 critical Solana vulnerability patterns. Each crate follows a `mod insecure` vs `mod secure` pattern to facilitate side-by-side diffing and learning.

### 2. Performance Research Lab
A benchmarking environment using `mollusk-svm` to measure the Compute Unit (CU) and memory overhead of "Safe Anchor" abstractions versus optimized "Safe Pinocchio" implementations.

---

## 🛠 Vulnerability Matrix

| ID | Vulnerability | Category | Tech Focus | Implementation Goal |
|---|---|---|---|---|
| 01 | [Missing Signer](./programs/01-missing-signer) | Sanity | Anchor | Baseline validation check. |
| 02 | [Integer Overflow](./programs/02-arithmetic-overflow) | Core | Rust | `checked_add` vs standard operators. |
| 03 | [PDA Metadata Leak](./programs/03-pda-seed-leak) | Privacy | Systems | Salted Hash derivation for seeds. |
| 04 | [Zombie Resurrection](./programs/04-account-closing) | Memory | Pinocchio | Manual data zeroing vs closure. |
| 05 | [Type Confusion](./programs/05-type-confusion) | Safety | Pinocchio | 8-byte Discriminator verification. |
| 06 | [Unaligned Pointer](./programs/06-zero-copy-alignment) | Hardware | Pinocchio | Zero-copy alignment & casting. |
| 07 | [Hook Reentrancy](./programs/07-hook-reentrancy) | Complex | Token-22 | CEI Pattern in CPI logic. |

---

## 🏁 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (2024 Edition)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18.26 recommended)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.30.1)
- Node.js & Yarn

### Installation
```bash
git clone https://github.com/nathfavour/bytesUnchecked.git
cd bytesUnchecked
yarn install
```

### Build & Test
```bash
# Build all programs
cargo build

# Run integration tests (requires local validator or anchor test)
anchor test
```

### Documentation
The deep-dive explanations for every vulnerability are located in the `/docs` folder and served via Docusaurus.
```bash
cd docs
npm install
npm start
```

---

## 🛡 Security Notice
This repository contains intentionally vulnerable code for educational purposes. **NEVER** use the `insecure` modules in a production environment. Always refer to the `secure` implementations and the CEI pattern for mainnet deployments.

## 📄 License
Distributed under the MIT License. See `LICENSE` for more information.
