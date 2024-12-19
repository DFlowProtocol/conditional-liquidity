use anchor_lang::prelude::*;

use crate::errors::SegmenterRegistryError;
use crate::state::*;

#[derive(Accounts)]
pub struct RemoveSegmenter<'info> {
    #[account(mut)]
    pub registry: AccountLoader<'info, Registry>,

    #[account(has_one = admin @ SegmenterRegistryError::InvalidAdminSpecified)]
    pub config: Account<'info, Config>,

    /// The admin for the deployment
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<RemoveSegmenter>, key: Pubkey) -> Result<()> {
    if ctx.accounts.registry.load_mut()?.remove(key).is_none() {
        msg!("Registry does not contain key: {}", key);
    };
    Ok(())
}
