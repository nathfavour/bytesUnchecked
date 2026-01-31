use anchor_lang::prelude::*;

declare_id!("H6S2mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8");

#[program]
pub mod vuln_missing_signer {
    use super::*;

    pub fn initialize_insecure(ctx: Context<InitializeInsecure>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn initialize_secure(ctx: Context<InitializeSecure>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn update_admin_insecure(ctx: Context<UpdateAdminInsecure>, new_admin: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // VULNERABILITY: Missing .is_signer check on the admin account.
        // Anyone can pass the admin's public key without proving they own it.
        state.admin = new_admin;
        Ok(())
    }

    pub fn update_admin_secure(ctx: Context<UpdateAdminSecure>, new_admin: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // SECURE: Anchor's Signer<'info> type automatically enforces the is_signer check.
        state.admin = new_admin;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeInsecure<'info> {
    #[account(init, payer = payer, space = 8 + 32)]
    pub state: Account<'info, AdminState>,
    /// CHECK: This is fine for initialization
    pub admin: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeSecure<'info> {
    #[account(init, payer = payer, space = 8 + 32)]
    pub state: Account<'info, AdminState>,
    pub admin: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdminInsecure<'info> {
    #[account(mut)]
    pub state: Account<'info, AdminState>,
    /// CHECK: This is insecure because we don't check if this account signed the transaction.
    /// In a raw Solana program, we'd check `admin.is_signer`.
    /// Here, using UncheckedAccount without a manual check is the vulnerability.
    pub admin: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateAdminSecure<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, AdminState>,
    pub admin: Signer<'info>,
}

#[account]
pub struct AdminState {
    pub admin: Pubkey,
}