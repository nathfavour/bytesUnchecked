# 05 - Type Confusion

## Analysis
Type confusion occurs when a program interprets an account of one type as if it were another. In Solana, this usually happens when a program accepts two different account types with similar memory layouts but different roles (e.g., `User` vs. `Admin`).

Without a **Discriminator** (a unique prefix identifying the type), an attacker can pass a `User` account into an instruction expecting an `Admin` account, potentially gaining unauthorized privileges.

## Exploit (Insecure)
Using `UncheckedAccount` and manually deserializing without checking a discriminator:

```rust
pub fn admin_action_insecure(ctx: Context<AdminActionInsecure>) -> Result<()> {
    // VULNERABILITY: No check to ensure 'admin' isn't actually a 'user' account.
    msg!("Privileged action by: {}", ctx.accounts.admin.key());
    Ok(())
}
```

## Fix (Secure)
The standard solution is to prepend an 8-byte discriminator to every account. Anchor does this by default for any struct marked with `#[account]`.

```rust
pub fn admin_action_secure(ctx: Context<AdminActionSecure>) -> Result<()> {
    // SECURE: Anchor checks the 8-byte discriminator automatically.
    msg!("Privileged action by: {}", ctx.accounts.admin.authority);
    Ok(())
}
```

## Benchmarks
| Implementation | CU Cost | Delta |
|---|---|---|
| Unchecked | ~150 | Baseline |
| Discriminator Check | ~450 | +300 CU |

*Note: The cost includes reading and comparing the first 8 bytes of the account data.*