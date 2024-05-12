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


#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy)]
pub struct PackageInitParams{
    pub id: u16,
    pub price: u32, //4 price in USDT 4 decimals
}



pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [u8]],
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}



