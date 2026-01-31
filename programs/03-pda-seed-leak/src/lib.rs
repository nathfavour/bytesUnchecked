use anchor_lang::prelude::*;
use solana_program::hash::hash;

declare_id!("E765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_pda_seed_leak {
    use super::*;

    pub fn initialize_insecure(ctx: Context<InitializeInsecure>, sensitive_id: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.sensitive_id = sensitive_id;
        Ok(())
    }

    pub fn initialize_secure(ctx: Context<InitializeSecure>, sensitive_id: String) -> Result<()> {
        let profile = &mut ctx.accounts.profile;
        profile.sensitive_id = sensitive_id;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(sensitive_id: String)]
pub struct InitializeInsecure<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + sensitive_id.len(),
        // VULNERABILITY: Using sensitive_id directly as a seed leaks it to anyone 
        // who can observe the transaction or the account derivation.
        seeds = [b"profile", sensitive_id.as_bytes()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(sensitive_id: String)]
pub struct InitializeSecure<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 4 + sensitive_id.len(),
        // SECURE: Use a hash of the sensitive_id to obfuscate it.
        // This prevents the raw ID from being visible in the PDA seeds.
        seeds = [b"profile", hash(sensitive_id.as_bytes()).to_bytes().as_ref()],
        bump
    )]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserProfile {
    pub sensitive_id: String,
}