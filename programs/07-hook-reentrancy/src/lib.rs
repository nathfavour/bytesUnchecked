use anchor_lang::prelude::*;

declare_id!("J765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_hook_reentrancy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance = 1000;
        Ok(())
    }

    pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        if vault.balance < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        // Potential reentrancy: Interaction before effect
        msg!("Transferring {} lamports...", amount);
        
        vault.balance -= amount;
        Ok(())
    }

    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        if vault.balance < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        // CEI Pattern: Effect before interaction
        vault.balance -= amount;

        msg!("Transferring {} lamports safely...", amount);
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
pub struct WithdrawInsecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub destination: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub destination: UncheckedAccount<'info>,
}

#[account]
pub struct Vault {
    pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds in vault.")]
    InsufficientFunds,
}
