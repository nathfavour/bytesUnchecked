# BytesUnchecked

[![CI](https://github.com/nathfavour/bytesUnchecked/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/nathfavour/bytesUnchecked/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**BytesUnchecked** is a collection of Solana program vulnerabilities and their secure implementations. It serves as a reference for developers to learn about common pitfalls and how to use the Anchor framework to mitigate them.

## 🚀 Features

- **7 Vulnerability Patterns**: Examples ranging from basic signer checks to complex reentrancy.
- **Side-by-Side Comparison**: Every program includes `insecure` and `secure` modules.
- **Integration Tests**: TypeScript tests that verify the vulnerabilities can be exploited and that the fixes work.

## 🛠 Vulnerability List

| ID | Vulnerability | Category |
|---|---|---|
| 01 | [Missing Signer](./programs/01-missing-signer) | Signature Verification |
| 02 | [Integer Overflow](./programs/02-arithmetic-overflow) | Arithmetic |
| 03 | [PDA Seed Leak](./programs/03-pda-seed-leak) | PDA Security |
| 04 | [Account Closing](./programs/04-account-closing) | Memory Management |
| 05 | [Type Confusion](./programs/05-type-confusion) | Account Validation |
| 06 | [Zero-Copy Alignment](./programs/06-zero-copy-alignment) | Memory Layout |
| 07 | [Hook Reentrancy](./programs/07-hook-reentrancy) | CPI Logic |

## 🏁 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- Node.js & Yarn

### Installation
```bash
yarn install
```

### Build & Test
```bash
anchor build
anchor test
```

## 🛡 Security Notice
This repository contains intentionally vulnerable code. **DO NOT** use the `insecure` modules in production.

## 📄 License
MIT