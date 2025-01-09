# Solana Conditional Liquidity

This crate contains tooling for Solana DEXs to support conditional liquidity. The primary functions
it exposes are `is_invoked_by_segmenter` and `verify_origin`, which can be used to branch based on
whether an instruction invocation was signed by a segmenter and based on the application or frontend
that originated the invocation.

### `is_invoked_by_segmenter`
Checks whether the invocation was signed by a segmenter. Use this if you don't need to branch on the
origin of the invocation.
```rs
use solana_conditional_liquidity::is_invoked_by_segmenter;
use solana_program::account_info::AccountInfo;

fn handler(registry: &AccountInfo<'_>, segmenter: &AccountInfo<'_>) {
    if !is_invoked_by_segmenter(registry, segmenter) {
        // The invocation wasn't signed by a segmenter
        return;
    }

    // Do stuff that you only allow when the invocation was signed by a segmenter
    // ...
}
```

### `verify_origin`
Checks whether the invocation was signed by a segmenter and verifies the origin of the
invocation. Use this if you want to branch on the origin of the invocation.
```rs
use solana_conditional_liquidity::{origin_identity, verify_origin, Origin, VerifyOriginResult};
use solana_program::account_info::AccountInfo;

fn handler(
    registry: &AccountInfo<'_>,
    segmenter: &AccountInfo<'_>,
    claimed_origin: Origin,
) {
    let VerifyOriginResult::InvokedBySegmenter(origin) =
        verify_origin(claimed_origin, registry, segmenter)
    else {
        // The invocation wasn't signed by a segmenter
        return;
    };

    // Branch on the origin here
    match origin {
        // Add special logic for specific origins here...
        origin_identity::UNKNOWN => {
            // Origin is unknown or could not be verified
        }
        // These identifiers are just examples, but you get the idea...
        // origin_identity::DFLOW_MOBILE => {}
        // origin_identity::DFLOW_WEB => {}
        // origin_identity::PHANTOM_MOBILE => {}
        // origin_identity::PHANTOM_EXTENSION => {}
        // origin_identity::SOLFLARE_MOBILE => {}
        // origin_identity::SOLFLARE_EXTENSION => {}
        _ => {
            // Fallback for any other origin. This is different than the origin being unknown.
        }
    };
}
```
