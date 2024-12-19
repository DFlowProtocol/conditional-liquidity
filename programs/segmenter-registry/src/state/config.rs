use anchor_lang::prelude::*;

/// The global config for the registry.
#[account]
pub struct Config {
    /// The public key of the account that has permission to modify the Registry accounts.
    pub admin: Pubkey,
}

impl Config {
    pub const LEN: usize = 8 + 32;
    pub const SEED: &'static [u8] = b"config";
}
