use anchor_lang::prelude::*;

use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("SRegZsVZDDqwc7W5iMUSsmKNnXzgfczKzFpimRp5iWw");

#[program]
pub mod segmenter_registry {
    use super::*;

    /// Initializes the deployment with an admin
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    /// Creates a new registry
    pub fn create_registry(ctx: Context<CreateRegistry>) -> Result<()> {
        instructions::create_registry::handler(ctx)
    }

    /// Allows the admin to add a new segmenter to a registry
    pub fn add_segmenter(ctx: Context<AddSegmenter>, new_segmenter: Pubkey) -> Result<()> {
        instructions::add_segmenter::handler(ctx, new_segmenter)
    }

    /// Allows the admin to remove an existing segmenter from a registry
    pub fn remove_segmenter(ctx: Context<RemoveSegmenter>, key: Pubkey) -> Result<()> {
        instructions::remove_segmenter::handler(ctx, key)
    }

    /// Allows the admin to appoint a new admin
    pub fn change_admin(ctx: Context<ChangeAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::change_admin::handler(ctx, new_admin)
    }
}
