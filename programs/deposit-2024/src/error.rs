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

    #[msg("Admin existed or max admin")]
    AdminAccountExisted,

    #[msg("Currency existed")]
    CurrencyExisted,

    #[msg("Package existed")]
    PackageExisted,

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

    #[msg("Input invalid")]
    InputInvalid,

    #[msg("Currency is not support")]
    CurrencyNotSupport,

    #[msg("Insufficient Amount")]
    InsufficientAmount


}

impl From<DepositErrors> for ProgramError {
    fn from(e: DepositErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}