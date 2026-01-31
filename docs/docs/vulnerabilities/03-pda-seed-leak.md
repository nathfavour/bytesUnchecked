# 03 - PDA Metadata Leak

## Analysis
Program Derived Addresses (PDAs) are derived using a collection of seeds. If sensitive information (like a user's Social Security Number, phone number, or real-world ID) is used directly as a seed, that information is leaked onto the public blockchain.

Even if the account data is encrypted, the **seeds are public**. Anyone scanning the blockchain can see the seeds used to derive every account.

## Exploit (Insecure)
Using raw PII (Personally Identifiable Information) as a seed:

```rust
pub struct InitializeInsecure<'info> {
    #[account(
        init,
        seeds = [b"profile", sensitive_id.as_bytes()], // VULNERABILITY: Plaintext leak
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    // ...
}
```

## Fix (Secure)
To protect privacy, sensitive seeds should be hashed before being passed to the PDA derivation logic. This allows the program to maintain a unique mapping for the user without revealing their actual ID.

```rust
pub struct InitializeSecure<'info> {
    #[account(
        init,
        // SECURE: Only the hash of the ID is visible publicly.
        seeds = [b"profile", hash(sensitive_id.as_bytes()).to_bytes().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    // ...
}
```

## Benchmarks
| Implementation | CU Cost | Delta |
|---|---|---|
| Raw Seed | ~800 | Baseline |
| Hashed Seed | ~1,400 | +600 CU |

*Note: Hashing (SHA-256) inside a program is compute-intensive but essential for privacy compliance.*