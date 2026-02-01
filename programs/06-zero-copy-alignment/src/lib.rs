use anchor_lang::prelude::*;
use bytemuck::{Pod, Zeroable};

declare_id!("Zcp1mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_zero_copy_alignment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut data = ctx.accounts.data.load_init()?;
        data.padding = 1;
        data.val = 0x1122334455667788;
        Ok(())
    }

    /// VULNERABLE: Performs raw pointer casting from a byte array without checking alignment.
    /// In Solana (especially on BPF/SBF hardware), loading a u64 from an unaligned address
    /// can cause a SIGBUS or return corrupted data depending on the VM version.
    pub fn update_insecure(ctx: Context<UpdateInsecure>, offset: u32) -> Result<()> {
        let account_info = &ctx.accounts.data;
        let data = account_info.try_borrow_data()?;
        
        // We manually calculate a pointer offset that might be misaligned
        unsafe {
            let base_ptr = data.as_ptr().add(offset as usize);
            
            // DANGEROUS: Casting to *const u64 without verifying that (base_ptr % 8 == 0)
            let misaligned_ptr = base_ptr as *const u64;
            
            // This load may fail or be unpredictable if misaligned
            let value = *misaligned_ptr;
            msg!("Read value from offset {}: {:x}", offset, value);
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
#[derive(Pod, Zeroable)]
pub struct BigData {
    pub padding: u8,   // Offset 0
    // The compiler will add 7 bytes of padding here because 'val' must be 8-byte aligned
    pub val: u64,       // Offset 8
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<BigData>())]
    pub data: AccountLoader<'info, BigData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateInsecure<'info> {
    /// CHECK: We are intentionally doing unsafe raw data access
    pub data: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateSecure<'info> {
    #[account(mut)]
    pub data: AccountLoader<'info, BigData>,
}