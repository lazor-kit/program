use anchor_lang::error_code;

/// Custom errors for the Lazor Kit program
#[error_code]
pub enum LazorKitError {
    /// Authentication errors
    #[msg("Authenticator passkey does not match argument passkey")]
    InvalidPasskey,
    #[msg("Authenticator does not match smart wallet")]
    InvalidAuthenticator,
    #[msg("Invalid rule program for operation")]
    InvalidRuleProgram,
    #[msg("Whitelist does not contain program key")]
    ProgramNotInWhitelist,
    /// Secp256r1 verification errors
    #[msg("Invalid Secp256r1 program")]
    InvalidSecp256r1Program,
    #[msg("No accounts should be passed for signature verification")]
    InvalidSecp256r1VerifyAccounts,
    #[msg("Invalid instruction length for signature verification")]
    InvalidSecp256r1VerifyData,
    #[msg("Signature header verification failed")]
    VerifyHeaderMismatchError,
    #[msg("Signature data verification failed")]
    VerifyDataMismatchError,
    /// Account validation errors
    #[msg("Invalid or missing required account")]
    InvalidAccountInput,
    #[msg("Invalid rule instruction provided")]
    InvalidRuleInstruction,
}
