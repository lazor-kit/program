use anchor_lang::prelude::*;

/// Custom error codes
#[error_code]
pub enum ContractError {
    #[msg("Signature verification failed.")]
    SigVerificationFailed,

    #[msg("Too many public keys.")]
    TooManyPubkey,

    #[msg("Invalid pubkey.")]
    InvalidPubkey,
}
