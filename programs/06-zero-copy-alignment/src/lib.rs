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

    pub fn update_insecure(ctx: Context<UpdateInsecure>, _val: u64) -> Result<()> {
        let account_info = &ctx.accounts.data;
        let data = account_info.try_borrow_data()?;
        
        // Unsafe cast without alignment check
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

#[account(zero_copy)]
#[repr(C)]
// #[derive(Pod, Zeroable)] // Removed because #[account(zero_copy)] already implements them
pub struct BigData {
    pub val: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
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