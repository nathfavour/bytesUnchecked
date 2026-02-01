use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("Ad2Z7mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

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
        // We actually execute a CPI to the external program provided.
        // In a real attack, this program would call 'withdraw_insecure' again.
        let ix = Instruction {
            program_id: ctx.accounts.external_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.vault.key(), false),
            ],
            data: vec![1, 2, 3], // Dummy data
        };

        invoke(
            &ix,
            &[
                ctx.accounts.external_program.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;
        
        // Re-fetch vault balance because it might have changed during CPI
        // (Though in a real exploit, the subtraction below is what's dangerous)
        vault.balance = vault.balance.checked_sub(amount).ok_or(error!(ErrorCode::InsufficientFunds))?;
        
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

        let ix = Instruction {
            program_id: ctx.accounts.external_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.vault.key(), false),
            ],
            data: vec![1, 2, 3],
        };

        invoke(
            &ix,
            &[
                ctx.accounts.external_program.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        )?;

        msg!("Withdrawal successful. New balance: {}", vault.balance);
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
