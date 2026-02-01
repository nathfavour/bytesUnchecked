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

        // VULNERABLE: Only transferring lamports does NOT close the account in a way
        // that prevents it from being used again in the same transaction or 
        // before the end of the slot if not careful. 
        // More importantly, it doesn't clear the data/discriminator properly.
        let dest_lamports = dest.lamports();
        **dest.lamports.borrow_mut() = dest_lamports.checked_add(vault.lamports()).unwrap();
        **vault.lamports.borrow_mut() = 0;
        
        Ok(())
    }

    pub fn close_secure(ctx: Context<CloseSecure>) -> Result<()> {
        // SECURE: The 'close = destination' attribute in the Accounts struct
        // handles lamport transfer AND clears the account discriminator/data.
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
    // The 'close' attribute is the idiomatic and safe way to close accounts in Anchor.
    #[account(mut, close = destination)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub destination: Signer<'info>,
}

#[account]
pub struct Vault {
    pub data: u64,
}