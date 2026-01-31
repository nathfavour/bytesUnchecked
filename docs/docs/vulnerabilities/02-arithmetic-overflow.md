# 02 - Arithmetic Overflow

## Analysis
Arithmetic overflows and underflows occur when a mathematical operation results in a value that exceeds the maximum or minimum limit of the data type (e.g., `u64`). In older versions of Rust or when `overflow-checks` are disabled, these operations "wrap around."

In a financial context, an underflow (e.g., `0 - 1`) could result in a massive balance (`2^64 - 1`), while an overflow (e.g., `max + 1`) could reset a user's funds to zero.

## Exploit (Insecure)
The insecure version uses the standard addition operator `+`. While modern Anchor projects usually enable overflow checks by default in `Cargo.toml`, relying on compiler flags rather than explicit logic is dangerous.

```rust
pub fn deposit_insecure(ctx: Context<UpdateVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    //  If overflow-checks are off, this wraps.
    // If they are on, it panics the program, which is better but still unhandled.
    vault.balance += amount;
    Ok(())
}
```

## Fix (Secure)
The secure implementation uses Rust's `checked_*` methods or Anchor's `math` features to handle errors gracefully.

```rust
pub fn deposit_secure(ctx: Context<UpdateVault>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    //  Use checked_add to return an explicit error on overflow.
    vault.balance = vault.balance.checked_add(amount)
        .ok_or(error!(ErrorCode::Overflow))?;
    Ok(())
}
```

