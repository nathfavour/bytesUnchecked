use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("G765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

#[program]
pub mod vuln_type_confusion {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.discriminator = User::DISCRIMINATOR;
        user.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.discriminator = Admin::DISCRIMINATOR;
        admin.authority = ctx.accounts.authority.key();
        Ok(())
    }

    /// This instruction is supposed to be admin-only.
    pub fn admin_action_insecure(ctx: Context<AdminActionInsecure>) -> Result<()> {
        // VULNERABILITY: We don't check if the account is actually an Admin account.
        // If a User account is passed, it might be interpreted as an Admin account
        // because they have the same memory layout (excluding discriminator which we ignore here).
        msg!("Admin action performed by: {}", ctx.accounts.admin.authority);
        Ok(())
    }

    pub fn admin_action_secure(ctx: Context<AdminActionSecure>) -> Result<()> {
        // SECURE: Anchor handles discriminator checks automatically for Account<'info, T>.
        // In raw Pinocchio, we would manually check:
        // if account.data[0..8] != Admin::DISCRIMINATOR { return Err(TypeConfusion); }
        msg!("Admin action performed securely by: {}", ctx.accounts.admin.authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub admin: Account<'info, Admin>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminActionInsecure<'info> {
    /// CHECK: We are intentionally not using Account<'info, Admin> to simulate 
    /// a missing type check or a manual deserialization vulnerability.
    pub admin: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminActionSecure<'info> {
    pub admin: Account<'info, Admin>,
    pub authority: Signer<'info>,
}

#[account]
pub struct User {
    pub discriminator: u64,
    pub authority: Pubkey,
}

impl User {
    pub const DISCRIMINATOR: u64 = 1;
}

#[account]
pub struct Admin {
    pub discriminator: u64,
    pub authority: Pubkey,
}

impl Admin {
    pub const DISCRIMINATOR: u64 = 2;
}