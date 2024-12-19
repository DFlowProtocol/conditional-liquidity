use anchor_lang::prelude::*;

use crate::errors::SegmenterRegistryError;

const MAX_ITEMS: usize = 64;

/// The registry account stores a mapping of registered segmenter accounts
#[account(zero_copy)]
pub struct Registry {
    pub registered_segmenters: [Pubkey; MAX_ITEMS],
}

impl Registry {
    pub const LEN: usize = 8 + std::mem::size_of::<Self>();
    pub const SEED: &'static [u8] = b"registry";

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            registered_segmenters: [Pubkey::default(); MAX_ITEMS],
        }
    }

    pub fn add(&mut self, key: Pubkey) -> Result<()> {
        if self.is_segmenter_registered(&key) {
            return Err(error!(SegmenterRegistryError::DuplicateEntry));
        }

        let Some(insert_idx) = self
            .registered_segmenters
            .iter()
            .position(|item| item == &Pubkey::default())
        else {
            return Err(error!(SegmenterRegistryError::RegistrySaturated));
        };

        self.registered_segmenters[insert_idx] = key;
        self.registered_segmenters.sort_unstable();
        Ok(())
    }

    pub fn is_segmenter_registered(&self, key: &Pubkey) -> bool {
        self.registered_segmenters.binary_search(key).is_ok()
    }

    pub fn remove(&mut self, key: Pubkey) -> Option<Pubkey> {
        let maybe_idx = self.registered_segmenters.binary_search(&key);
        if let Ok(idx) = maybe_idx {
            self.registered_segmenters[idx] = Pubkey::default();
            self.registered_segmenters.sort_unstable();
            Some(key)
        } else {
            None
        }
    }
}
