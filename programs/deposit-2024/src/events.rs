use anchor_lang::prelude::*;

use crate::{AuthRole, DepositStatus};

#[event]
pub struct DepositEvent {
    pub token: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub time: i64,
}

#[event]
pub struct SetAuthorityEvent {
    pub admin: Pubkey,
    pub role: AuthRole,
    pub operators: Vec<Pubkey>,
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
    pub packages: Vec<u64>,
    pub time: i64,
}
