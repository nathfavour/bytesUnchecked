use anchor_lang::prelude::*;

declare_id!("C765mYyvXQW8vXQW8vXQW8vXQW8vXQW8vXQW8vXQW8v");

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
        // VULNERABLE: 'admin' is an UncheckedAccount and we never check is_signer
        // An attacker can provide any admin pubkey and the program will accept it.
        state.admin = new_admin;
        Ok(())
    }

    pub fn update_admin_secure(ctx: Context<UpdateAdminSecure>, new_admin: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        // SECURE: 'admin' is a Signer<'info>, Anchor automatically checks ctx.accounts.admin.is_signer
        state.admin = new_admin;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeInsecure<'info> {
    #[account(init, payer = payer, space = 8 + 32)]
    pub state: Account<'info, AdminState>,
    /// CHECK: Initialization check
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
    /// CHECK: Missing signature verification
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