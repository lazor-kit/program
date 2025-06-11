use anchor_lang::error_code;

#[error_code]
pub enum DefaultRuleError {
    #[msg("Authenticator does not match rule admin")]
    InvalidAuthenticator,

    #[msg("Invalid smart wallet")]
    InvalidSmartWallet,
}
