use anchor_lang::prelude::*;
use std::ops::DerefMut;

declare_id!("F765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_account_closing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.data = 100;
        Ok(())
    }

    pub fn close_insecure(ctx: Context<CloseInsecure>) -> Result<()> {
        let dest = ctx.accounts.destination.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();

        // Transfer lamports without clearing data
        let dest_lamports = dest.lamports();
        **dest.lamports.borrow_mut() = dest_lamports.checked_add(vault.lamports()).unwrap();
        **vault.lamports.borrow_mut() = 0;
        
        Ok(())
    }

    pub fn close_secure(ctx: Context<CloseSecure>) -> Result<()> {
        let dest = ctx.accounts.destination.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();

        // Clear data and transfer lamports
        let dest_lamports = dest.lamports();
        **dest.lamports.borrow_mut() = dest_lamports.checked_add(vault.lamports()).unwrap();
        **vault.lamports.borrow_mut() = 0;

        let mut data = vault.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseInsecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseSecure<'info> {
    #[account(mut, close = destination)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: Signer<'info>,
}

#[account]
pub struct Vault {
    pub data: u64,
}