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

        // Interaction BEFORE Effect (Vulnerable)
        // In a real scenario, this would be a CPI to a program that could call back (re-enter)
        // such as a Token-2022 Transfer Hook or a malicious callback.
        msg!("Interaction: Calling external program...");
        let _ = invoke_placeholder_callback(
            &ctx.accounts.external_program.to_account_info(),
            &ctx.accounts.vault.to_account_info()
        )?;
        
        vault.balance -= amount;
        msg!("Withdrawal successful. New balance: {}", vault.balance);
        Ok(())
    }

    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        if vault.balance < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        // Effect BEFORE Interaction (Secure - CEI Pattern)
        vault.balance -= amount;

        msg!("Interaction: Calling external program safely...");
        let _ = invoke_placeholder_callback(
            &ctx.accounts.external_program.to_account_info(),
            &ctx.accounts.vault.to_account_info()
        )?;

        msg!("Withdrawal successful. New balance: {}", vault.balance);
        Ok(())
    }
}

/// A placeholder for a CPI call that could trigger a reentrancy.
fn invoke_placeholder_callback<'info>(
    _program: &AccountInfo<'info>,
    _vault: &AccountInfo<'info>,
) -> Result<()> {
    // In a real exploit, this would be:
    // anchor_lang::solana_program::program::invoke(...)
    Ok(())
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
    /// CHECK: Placeholder for an external program (e.g., a token hook)
    pub external_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Placeholder for an external program
    pub external_program: UncheckedAccount<'info>,
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
