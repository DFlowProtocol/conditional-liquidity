use solana_program::{account_info::AccountInfo, pubkey, pubkey::Pubkey};

use crate::is_invoked_by_segmenter;

/// The identifier for the origin of an invocation. This identifies the application or frontend from
/// which the invocation originated.
pub type Origin = u16;

pub mod origin_identity {
    use super::Origin;

    /// Origin when the application or frontend is unknown or could not be verified
    pub const UNKNOWN: Origin = 0;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyOriginResult {
    /// The invocation was signed by a segmenter
    InvokedBySegmenter(Origin),
    /// The invocation was not signed by a segmenter
    NotInvokedBySegmenter,
}

/// Checks whether the invocation was signed by a segmenter and verifies the origin of the
/// invocation. Use this if you want to branch on the origin of the invocation.
///
/// Examples
///
/// ```
/// use solana_conditional_liquidity::{origin_identity, verify_origin, Origin, VerifyOriginResult};
/// use solana_program::account_info::AccountInfo;
///
/// fn handler(
///     registry: &AccountInfo<'_>,
///     segmenter: &AccountInfo<'_>,
///     claimed_origin: Origin,
/// ) {
///     let VerifyOriginResult::InvokedBySegmenter(origin) =
///         verify_origin(claimed_origin, registry, segmenter)
///     else {
///         // The invocation wasn't signed by a segmenter
///         return;
///     };
///
///     // Branch on the origin here
///     match origin {
///         // Add special logic for specific origins here...
///         origin_identity::UNKNOWN => {
///             // Origin is unknown or could not be verified
///         }
///         // These identifiers are just examples, but you get the idea...
///         // origin_identity::DFLOW_MOBILE => {}
///         // origin_identity::DFLOW_WEB => {}
///         // origin_identity::PHANTOM_MOBILE => {}
///         // origin_identity::PHANTOM_EXTENSION => {}
///         // origin_identity::SOLFLARE_MOBILE => {}
///         // origin_identity::SOLFLARE_EXTENSION => {}
///         _ => {
///             // Fallback for any other origin. This is different than the origin being unknown.
///         }
///     };
/// }
/// ```
pub fn verify_origin(
    claimed_origin: Origin,
    registry: &AccountInfo<'_>,
    segmenter: &AccountInfo<'_>,
) -> VerifyOriginResult {
    if !is_invoked_by_segmenter(registry, segmenter) {
        return VerifyOriginResult::NotInvokedBySegmenter;
    }

    if registry.key != &ORIGIN_VERIFYING_SEGMENTER_REGISTRY {
        return VerifyOriginResult::InvokedBySegmenter(origin_identity::UNKNOWN);
    }

    // If one of the DFlow segmenters signed the invocation, we can trust the origin
    VerifyOriginResult::InvokedBySegmenter(claimed_origin)
}

/// The origin-verifying registry is a special registry that contains segmenters that verify the
/// origin of the invocation
pub const ORIGIN_VERIFYING_SEGMENTER_REGISTRY: Pubkey =
    pubkey!("Reg1Y127DNKYUTf3LinfEs3oiSiywJsyAobJMjqYqDE");

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use base64::Engine;
    use solana_program::system_program;

    use crate::Registry;

    use super::*;

    #[test]
    fn test_verify_origin() {
        let mut registry_data = base64::engine::general_purpose::STANDARD
                .decode("L65u9ri2/NoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADmpy9pZ0kSP2HFqsTSMjxFxburQjxQioc8A4BaVKbWtBAWsWrJs+pByyagNT2RTZ5E/wZAQB0FNhQpY/WTUev0FyJOST1AtAGCROxwJ16TmEm/91X11Lmzcymw9zcmibQnqAzYRp+mQPsmo+2htSt8O7nGFhRpTDQdq9qCtx9jA==")
                .unwrap();

        // Registry is the origin-verifying registry
        let mut registry_lamports = 15200640;
        let registry = AccountInfo {
            key: &pubkey!("Reg1Y127DNKYUTf3LinfEs3oiSiywJsyAobJMjqYqDE"),
            is_signer: false,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut registry_lamports)),
            data: Rc::new(RefCell::new(&mut registry_data)),
            owner: &Registry::PROGRAM_ID,
            executable: false,
            rent_epoch: 18446744073709551615,
        };

        // Signed and exists in origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &pubkey!("F2Me9XknvkPYjvoEgTXXKqARg58Ezo6ZmGhpYdS3UTmF"),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::InvokedBySegmenter(1)
        );

        // Didn't sign but exists in origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &pubkey!("F2Me9XknvkPYjvoEgTXXKqARg58Ezo6ZmGhpYdS3UTmF"),
            is_signer: false,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::NotInvokedBySegmenter
        );

        // Signed but doesn't exist in origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::NotInvokedBySegmenter
        );
    }

    #[test]
    fn test_verify_origin_non_verifying_registry() {
        let mut registry_data = base64::engine::general_purpose::STANDARD
                .decode("L65u9ri2/NoAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADmpy9pZ0kSP2HFqsTSMjxFxburQjxQioc8A4BaVKbWtBAWsWrJs+pByyagNT2RTZ5E/wZAQB0FNhQpY/WTUev0FyJOST1AtAGCROxwJ16TmEm/91X11Lmzcymw9zcmibQnqAzYRp+mQPsmo+2htSt8O7nGFhRpTDQdq9qCtx9jA==")
                .unwrap();

        // Registry is valid but isn't the origin-verifying registry
        let mut registry_lamports = 15200640;
        let registry = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: false,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut registry_lamports)),
            data: Rc::new(RefCell::new(&mut registry_data)),
            owner: &Registry::PROGRAM_ID,
            executable: false,
            rent_epoch: 18446744073709551615,
        };

        // Segmenter signed and exists in non-origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &pubkey!("F2Me9XknvkPYjvoEgTXXKqARg58Ezo6ZmGhpYdS3UTmF"),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::InvokedBySegmenter(origin_identity::UNKNOWN)
        );

        // Segmenter didn't sign but exists in non-origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &pubkey!("F2Me9XknvkPYjvoEgTXXKqARg58Ezo6ZmGhpYdS3UTmF"),
            is_signer: false,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::NotInvokedBySegmenter
        );

        // Segmenter signed but doesn't exist in non-origin-verifying registry
        let mut segmenter_lamports = 0;
        let mut segmenter_data = [];
        let segmenter = AccountInfo {
            key: &Pubkey::new_unique(),
            is_signer: true,
            is_writable: false,
            lamports: Rc::new(RefCell::new(&mut segmenter_lamports)),
            data: Rc::new(RefCell::new(&mut segmenter_data)),
            owner: &system_program::id(),
            executable: false,
            rent_epoch: 18446744073709551615,
        };
        assert_eq!(
            verify_origin(1, &registry, &segmenter),
            VerifyOriginResult::NotInvokedBySegmenter
        );
    }
}
