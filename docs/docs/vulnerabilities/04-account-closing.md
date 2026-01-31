# 04 - Zombie Resurrection (Account Closing)

## Analysis
When a Solana account is "closed," its lamports are transferred to a destination account. However, if the program logic doesn't explicitly **zero out the data**, the bytes remain in the account's memory space until the runtime purges it.

If an attacker manages to re-fund the account with lamports before the data is purged, the program might interpret the "zombie" data as valid state, leading to logic errors.

## Exploit (Insecure)
Manually transferring lamports without cleaning the memory:

```rust
pub fn close_insecure(ctx: Context<CloseInsecure>) -> Result<()> {
    let dest = ctx.accounts.destination.to_account_info();
    let vault = ctx.accounts.vault.to_account_info();

    let dest_lamports = dest.lamports();
    **dest.lamports.borrow_mut() = dest_lamports + vault.lamports();
    **vault.lamports.borrow_mut() = 0; // Data still exists in memory!
    Ok(())
}
```

## Fix (Secure)
Properly closing an account requires two steps:
1. Transferring all lamports.
2. Overwriting the data with zeros.

Anchor's `close = destination` constraint handles this automatically and securely.

```rust
pub fn close_secure(ctx: Context<CloseSecure>) -> Result<()> {
    // Anchor automatically zeros data and transfers lamports
    Ok(())
}
```

