use anchor_lang::prelude::*;

declare_id!("D765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_arithmetic_overflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance = initial_balance;
        Ok(())
    }

    pub fn deposit_insecure(ctx: Context<UpdateVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // VULNERABILITY: Standard addition operator '+' will wrap in release mode
        // if overflow-checks are disabled, or panic if enabled.
        // Even with checks enabled, not using checked_add is considered poor practice
        // as it doesn't allow for graceful error handling.
        vault.balance += amount;
        Ok(())
    }

    pub fn deposit_secure(ctx: Context<UpdateVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        // SECURE: Use checked_add to handle overflows explicitly.
        vault.balance = vault.balance.checked_add(amount).ok_or(error!(ErrorCode::Overflow))?;
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
pub struct UpdateVault<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow occurred.")]
    Overflow,
}