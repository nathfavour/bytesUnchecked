use anchor_lang::prelude::*;

declare_id!("8RR34N7BHCmaD1FDEuB9R2XwnHsEtXy766vCdki4KnFN");

#[program]
pub mod vuln_type_confusion {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin = &mut ctx.accounts.admin;
        admin.authority = ctx.accounts.authority.key();
        Ok(())
    }

    /// This instruction is supposed to be admin-only.
    /// VULNERABLE: It doesn't check if the 'admin' account is actually an Admin type.
    pub fn admin_action_insecure(ctx: Context<AdminActionInsecure>) -> Result<()> {
        let admin_info = &ctx.accounts.admin;
        
        // Manual deserialization without discriminator check!
        // An attacker can pass a 'User' account here.
        let data = admin_info.try_borrow_data()?;
        let mut data_ptr = &data[8..]; // Skip Anchor discriminator
        let admin_data = Admin::deserialize(&mut data_ptr)?;

        msg!("Admin action performed for authority: {}", admin_data.authority);
        Ok(())
    }

    pub fn admin_action_secure(ctx: Context<AdminActionSecure>) -> Result<()> {
        msg!("Admin action performed securely for: {}", ctx.accounts.admin.authority);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(init, payer = authority, space = 8 + 32)]
    pub admin: Account<'info, Admin>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminActionInsecure<'info> {
    /// CHECK: We are demonstrating a manual deserialization vulnerability
    pub admin: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminActionSecure<'info> {
    // Anchor's Account<'info, T> automatically checks the 8-byte discriminator
    pub admin: Account<'info, Admin>,
    pub authority: Signer<'info>,
}

#[account]
pub struct User {
    pub authority: Pubkey,
}

#[account]
pub struct Admin {
    pub authority: Pubkey,
}
