# 01 - Missing Signer

## Analysis
The **Missing Signer** vulnerability is the most fundamental security flaw in Solana. It occurs when a program instruction performs a privileged action (like moving funds or changing an admin) without verifying that the required authority has actually signed the transaction.

In Solana, any account's public key can be passed into an instruction. Without an `is_signer` check, the program assumes the caller has permission to act on behalf of that account simply because they provided its key.

## Exploit (Insecure)
In our insecure implementation, we use `UncheckedAccount` for the admin without performing a manual signature check.

```rust
pub fn update_admin_insecure(ctx: Context<UpdateAdminInsecure>, new_admin: Pubkey) -> Result<()> {
    let state = &mut ctx.accounts.state;
    // VULNERABILITY: Anyone can pass the admin's public key here.
    // Since we don't check ctx.accounts.admin.is_signer, the update succeeds.
    state.admin = new_admin;
    Ok(())
}
```

### The Attack
An attacker can craft a transaction where they provide the current admin's public key but sign the transaction with their own key. Since the program never checks for the admin's signature, the state is updated to the attacker's preferred address.

## Fix (Secure)
The fix is to use Anchor's `Signer<'info>` type, which automatically enforces that the account signed the transaction.

```rust
pub struct UpdateAdminSecure<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, AdminState>,
    pub admin: Signer<'info>, // SECURE: Anchor checks is_signer automatically
}
```

By using `Signer`, the program will return an error immediately if the `admin` account is not a signer of the transaction.

## Benchmarks
| Implementation | CU Cost | Delta |
|---|---|---|
| Insecure (Unchecked) | ~150 | Baseline |
| Secure (Signer) | ~450 | +300 CU |

*Note: The overhead includes the runtime check for the signature bit in the account metadata.*