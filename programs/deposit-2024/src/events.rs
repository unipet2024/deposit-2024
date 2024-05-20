use anchor_lang::prelude::*;

use crate::{AuthRole, DepositStatus};

#[event]
pub struct DepositEvent {
    pub package: u16,
    pub user: Pubkey,
    pub time: i64,
    pub amount: u64,
    pub token: String,
    pub slot: u64,
}

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub time: i64,
}

#[event]
pub struct ChangeOperatorWalletEvent {
    pub wallet: Pubkey,
    pub time: i64,
}

#[event]
pub struct SetStatusEvent {
    pub admin: Pubkey,
    pub status: DepositStatus,
    pub time: i64,
}

#[event]
pub struct SetPackageEvent {
    pub operator: Pubkey,
    pub token: Pubkey,
    pub time: i64,
}

#[event]
pub struct WithdrawEvent {
    pub address: Pubkey,
    pub amount: u64,
    pub time: i64,
}