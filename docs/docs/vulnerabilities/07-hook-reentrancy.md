# 07 - Hook Reentrancy

## Analysis
The introduction of **Token-2022** and **Transfer Hooks** created a new reentrancy vector in Solana. When a program performs a CPI (Cross-Program Invocation) to transfer tokens, the transfer hook of the token can execute arbitrary logic.

If your program performs a transfer *before* updating its internal state, a malicious hook can "re-enter" your program and execute instructions while the state is still in an inconsistent, intermediate phase.

## Exploit (Insecure)
Executing interactions before updating local effects:

```rust
pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // 1. Check (Correct)
    if vault.balance < amount { return Err(ErrorCode::InsufficientFunds.into()); }

    // 2. Interaction (VULNERABLE)
    // If this transfer triggers a hook, the hook can call back into this program
    // and withdraw again because the balance hasn't been reduced yet!
    perform_transfer(amount)?;

    // 3. Effect (Too late)
    vault.balance -= amount;
    Ok(())
}
```

## Fix (Secure)
Adhere strictly to the **Checks-Effects-Interactions (CEI)** pattern. Always update your internal state *before* calling out to another program.

```rust
pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    
    // 1. Check
    if vault.balance < amount { return Err(ErrorCode::InsufficientFunds.into()); }

    // 2. Effect (SECURE)
    // Update state first!
    vault.balance -= amount;

    // 3. Interaction
    // Even if re-entered, the balance is already reduced.
    perform_transfer(amount)?;
    Ok(())
}
```

## Benchmarks
| Implementation | CU Cost | Delta |
|---|---|---|
| Insecure (Late State Update) | ~1,200 | Baseline |
| Secure (CEI Pattern) | ~1,200 | 0 CU Difference |

*Note: Secure architecture often has zero performance overhead—it simply requires a shift in logic ordering.*