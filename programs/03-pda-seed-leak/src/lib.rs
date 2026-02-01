use anchor_lang::prelude::*;
use solana_program::hash::hash;

declare_id!("D7mu2Eyx2dyCoMZMgT55zRzRTjcuTNY9DpTWbbwEs5vo");

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
        // Sensitive data used as a seed
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
        // Hash sensitive data before using as a seed
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