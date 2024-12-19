use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = payer, space = Registry::LEN)]
    pub registry: AccountLoader<'info, Registry>,

    /// The payer for the registry account creation
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateRegistry>) -> Result<()> {
    *ctx.accounts.registry.load_init()? = Registry::new();
    Ok(())
}
