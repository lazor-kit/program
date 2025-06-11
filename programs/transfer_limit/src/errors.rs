use anchor_lang::error_code;

#[error_code]
pub enum TransferLimitError {
    #[msg("Invalid CPI data")]
    InvalidCpiData,

    #[msg("Member is not initialized")]
    MemberNotInitialized,

    #[msg("Amount exceeded limit in rule data")]
    TransferAmountExceedLimit,

    #[msg("Program must be system program, token program or token 2022 program")]
    InvalidProgram,
}
