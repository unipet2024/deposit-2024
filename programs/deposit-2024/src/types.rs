use anchor_lang::prelude::*;

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub enum AuthRole {
    Admin,
    Operator,
}

#[derive(PartialEq, Eq, AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
pub enum DepositStatus {
    Waiting,
    Open,
    Close,
}

// #[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
// pub struct TokenParms {
//     pub token: Vec<Pubkey>,
// }
