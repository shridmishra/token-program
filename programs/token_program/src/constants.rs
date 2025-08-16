//! Global constants (seeds, sizes, etc.)

/// PDA seed prefix for the mint authority PDA.
/// Full seeds: [MINT_AUTHORITY_SEED, mint_pubkey.as_ref()]
pub const MINT_AUTHORITY_SEED: &[u8] = b"mint_auth";
