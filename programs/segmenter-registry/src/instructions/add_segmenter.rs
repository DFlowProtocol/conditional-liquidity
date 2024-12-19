use anchor_lang::prelude::*;

use crate::errors::SegmenterRegistryError;
use crate::state::*;

#[derive(Accounts)]
pub struct AddSegmenter<'info> {
    #[account(mut)]
    pub registry: AccountLoader<'info, Registry>,

    #[account(has_one = admin @ SegmenterRegistryError::InvalidAdminSpecified)]
    pub config: Account<'info, Config>,

    /// The admin for the deployment
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<AddSegmenter>, new_segmenter: Pubkey) -> Result<()> {
    ctx.accounts.registry.load_mut()?.add(new_segmenter)?;
    Ok(())
}
