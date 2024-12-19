use anchor_lang::prelude::*;

use crate::errors::SegmenterRegistryError;
use crate::state::*;

#[derive(Accounts)]
pub struct ChangeAdmin<'info> {
    #[account(mut, has_one = admin @ SegmenterRegistryError::InvalidAdminSpecified)]
    pub config: Account<'info, Config>,

    /// The current admin
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
    ctx.accounts.config.admin = new_admin;

    Ok(())
}
