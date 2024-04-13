use anchor_lang::prelude::*;

#[error_code]
pub enum DepositErrors {
    #[msg("Package invalid")]
    PackageInvalid,

    #[msg("Token not support")]
    TokenNotSupport,

    #[msg("Deposit closed")]
    DepositClosed,

    #[msg("Admin account invalid")]
    AdminAccountInvalid,

    #[msg("Operator account invalid")]
    OperatorAccountInvalid,

    #[msg("Only admin")]
    OnlyAdmin,

    #[msg("Only Operator")]
    OnlyOperator,

    #[msg("Vault invalid")]
    VaultInvalid,

    #[msg("Operator not change")]
    OperatorNotChange,

    #[msg("InputInvalid")]
    InputInvalid,
}

impl From<DepositErrors> for ProgramError {
    fn from(e: DepositErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}