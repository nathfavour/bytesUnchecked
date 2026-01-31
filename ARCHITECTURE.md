# Architecture: bytesUnchecked

This project is a reference implementation for common Solana vulnerability patterns and their secure counterparts. It provides a side-by-side comparison of insecure code and the corresponding fix using the Anchor framework.

## 1. Project Structure

The repository is organized as a Rust Cargo Workspace. Each vulnerability is isolated into its own crate.

```text
bytesUnchecked/
├── programs/                      # Core vulnerability examples
│   ├── 01-missing-signer/         # Basic signature verification
│   ├── 02-arithmetic-overflow/    # Integer safety
│   ├── 03-pda-seed-leak/          # PDA derivation security
│   ├── 04-account-closing/        # Account data lifecycle
│   ├── 05-type-confusion/         # Account discriminator checks
│   ├── 06-zero-copy-alignment/    # Zero-copy and memory layout
│   └── 07-hook-reentrancy/        # Token-2022 transfer hooks
├── tests/                         # Integration tests (TypeScript)
├── docs/                          # Project documentation
├── Anchor.toml                    # Anchor configuration
└── Cargo.toml                     # Workspace configuration
```

## 2. Implementation Pattern

Each program module contains two sets of instructions:
- `_insecure`: Demonstrates the vulnerability.
- `_secure`: Demonstrates the corrected implementation using Anchor's built-in safety features.

## 3. Testing

Integration tests are located in the `tests/` directory and use the Anchor TS library to verify both the exploit and the fix.

To run tests:
```bash
anchor test
```

## 4. Documentation

Detailed explanations for each vulnerability pattern are available in the `docs/` folder.