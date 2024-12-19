use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = admin,
        space = Config::LEN,
        seeds = [
            Config::SEED,
        ],
        bump,
    )]
    pub config: Account<'info, Config>,

    /// The admin for the deployment
    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.config.admin = ctx.accounts.admin.key();
    Ok(())
}
