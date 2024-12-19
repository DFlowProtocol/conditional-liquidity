use anchor_lang::prelude::*;

#[error_code]
pub enum SegmenterRegistryError {
    #[msg("The registry is at capacity")]
    RegistrySaturated = 9000,
    #[msg("Segmenter already exists in the registry")]
    DuplicateEntry = 9001,
    #[msg("The admin specified is not authorized to invoke this instruction")]
    InvalidAdminSpecified = 9002,
}
