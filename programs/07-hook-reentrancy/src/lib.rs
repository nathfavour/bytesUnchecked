use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::instruction::Instruction;

declare_id!("BjVwjTm3TzEYN9uRZx78HDQ4g1kWSCWURMeJKuzfr8vY");

#[program]
pub mod vuln_hook_reentrancy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.balance = 1000;
        Ok(())
    }

    /// This is a malicious instruction that simulates a reentrant call.
    /// In a real attack, this would be in a SEPARATE program (e.g., a Token-2022 hook).
    pub fn malicious_callback(ctx: Context<MaliciousCallback>) -> Result<()> {
        msg!("MALICIOUS CALLBACK: Attempting reentrancy...");
        
        // We try to call back into the vault's withdraw function
        // We use a simple CPI here to keep it contained in one program for the demo.
        let ix = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.vault.key(), false),
                AccountMeta::new_readonly(crate::ID, false),
            ],
            data: anchor_lang::InstructionData::data(&crate::instruction::WithdrawInsecure { amount: 100 }),
        };

        invoke(
            &ix,
            &[
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.self_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    pub fn withdraw_insecure(ctx: Context<WithdrawInsecure>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        if vault.balance < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        msg!("WithdrawInsecure: Interaction starting (CPI)...");

        // Interaction BEFORE Effect (Vulnerable)
        let ix = Instruction {
            program_id: ctx.accounts.external_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.vault.key(), false),
                AccountMeta::new_readonly(crate::ID, false),
            ],
            data: anchor_lang::InstructionData::data(&crate::instruction::MaliciousCallback {}),
        };

        invoke(
            &ix,
            &[
                ctx.accounts.external_program.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.external_program.to_account_info(), // self
            ],
        )?;
        
        // REENTRANCY HAPPENS HERE:
        // The malicious_callback has already subtracted 100 from vault.balance.
        // But this instance of withdraw_insecure still has the OLD balance in memory
        // if we didn't reload it, OR it just proceeds to subtract again.
        
        vault.balance = vault.balance.checked_sub(amount).ok_or(error!(ErrorCode::InsufficientFunds))?;
        
        msg!("Withdrawal successful. New balance: {}", vault.balance);
        Ok(())
    }

    pub fn withdraw_secure(ctx: Context<WithdrawSecure>, amount: u64) -> Result<()> {
        // In a secure implementation, we use the CEI pattern or reentrancy guards.
        let vault = &mut ctx.accounts.vault;
        
        if vault.balance < amount {
            return Err(error!(ErrorCode::InsufficientFunds));
        }

        // Effect BEFORE Interaction
        vault.balance -= amount;

        msg!("WithdrawSecure: Interaction starting (CPI)...");
        // Even if we call a malicious program now, the balance is already updated.
        // If it tries to re-enter, it will see the reduced balance.

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
    /// CHECK: The program to call (can be ourselves for this demo)
    pub external_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawSecure<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: External program
    pub external_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct MaliciousCallback<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Self
    pub self_program: UncheckedAccount<'info>,
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