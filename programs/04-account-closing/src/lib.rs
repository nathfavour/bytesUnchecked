use anchor_lang::prelude::*;
use std::ops::DerefMut;

declare_id!("89m1e57mfnsPvqdKpYpSfRsDSXDWjg3Cky7MYb9P2dF6");

#[program]
pub mod vuln_account_closing {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.data = 1337;
        Ok(())
    }

    /// This instruction demonstrates that even if lamports are 0, the data remains.
    pub fn check_vault_data(ctx: Context<CheckVault>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        msg!("Vault data is still: {}", vault.data);
        if vault.data == 1337 {
            msg!("VULNERABILITY CONFIRMED: Account drained of lamports but data/discriminator still exists!");
        }
        Ok(())
    }

    pub fn close_insecure(ctx: Context<CloseInsecure>) -> Result<()> {
        let dest = ctx.accounts.destination.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();

        // VULNERABLE: Only transferring lamports does NOT clear the account data.
        // On Solana, an account is only 'closed' at the end of a transaction if its lamports are 0.
        // However, within the same transaction, another instruction can still read the 'dead' data.
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
pub struct CheckVault<'info> {
    pub vault: Account<'info, Vault>,
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