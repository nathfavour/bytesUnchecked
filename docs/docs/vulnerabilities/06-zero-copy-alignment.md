# 06 - Unaligned Pointer (Zero-Copy)

## Analysis
Zero-copy accounts allow programs to interact with account data directly in the Solana memory space without deserializing it into a new struct. This is highly efficient for large accounts.

However, zero-copy requires that the account data is **aligned** correctly in memory (usually to 8 bytes). Accessing unaligned data through a raw pointer in Rust can lead to program panics or undefined behavior.

## Exploit (Insecure)
Manually casting a data pointer without verifying alignment or using safe wrappers:

```rust
pub fn update_insecure(ctx: Context<UpdateInsecure>, _val: u64) -> Result<()> {
    let account_info = &ctx.accounts.data;
    let data = account_info.try_borrow_data()?;
    
    // VULNERABILITY: Unsafe cast. If data is not 8-byte aligned, this is UB.
    let ptr = data.as_ptr() as *const BigData;
    unsafe { msg!("Value: {}", (*ptr).val); }
    Ok(())
}
```

## Fix (Secure)
Use Anchor's `AccountLoader` and the `#[account(zero_copy)]` attribute. This ensures that the data is accessed through safe, alignment-aware wrappers.

```rust
#[account(zero_copy)]
#[repr(C)]
pub struct BigData {
    pub val: u64,
}

pub struct UpdateSecure<'info> {
    #[account(mut)]
    pub data: AccountLoader<'info, BigData>, // SECURE: Safe alignment handling
}
```

## Benchmarks
| Implementation | CU Cost | Delta |
|---|---|---|
| Standard Borsh | ~5,000+ | (For large accounts) |
| Zero-Copy | ~800 | -84% Savings |

*Note: Zero-copy is one of the most powerful performance optimizations in Solana, but it requires strict hardware-level safety.*