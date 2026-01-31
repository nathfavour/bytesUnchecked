use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("H765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_zero_copy_alignment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut data = ctx.accounts.data.load_init()?;
        data.val = 42;
        Ok(())
    }

    pub fn update_insecure(ctx: Context<UpdateInsecure>, val: u64) -> Result<()> {
        // VULNERABILITY: Manually casting data without alignment checks.
        // If the account data is not aligned to 8 bytes, this could panic.
        let account_info = &ctx.accounts.data;
        let data = account_info.try_borrow_data()?;
        
        // This is a dangerous cast if we don't know the alignment
        let ptr = data.as_ptr() as *const BigData;
        unsafe {
            msg!("Value: {}", (*ptr).val);
        }
        
        Ok(())
    }

    pub fn update_secure(ctx: Context<UpdateSecure>, val: u64) -> Result<()> {
        let mut data = ctx.accounts.data.load_mut()?;
        data.val = val;
        Ok(())
    }
}

#[zero_copy]
#[repr(C)]
#[derive(Pod, Zeroable)]
pub struct BigData {
    pub val: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero_copy, init, payer = user, space = 8 + 8)]
    pub data: AccountLoader<'info, BigData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateInsecure<'info> {
    /// CHECK: Manual deserialization
    pub data: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateSecure<'info> {
    #[account(mut)]
    pub data: AccountLoader<'info, BigData>,
}