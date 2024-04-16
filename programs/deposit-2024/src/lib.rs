use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use types::*;


declare_id!("3dHnbHNAVfx1u27VSoCszk3xbkrfVLXnP3BCxrp3AZju");

#[program]
pub mod deposit_2024 {
    use super::*;

    pub fn init(ctx: Context<InitDeposit>) -> Result<()> {
        init_deposit::init_handle(ctx)
    }

    pub fn set_packages(
        ctx: Context<SetPackage>,
        token: Pubkey,
        packages: Vec<PackageItem>,
        valid: bool,
    ) -> Result<()> {
        set_package::set_package_handle(ctx, token, packages, valid)
    }

    pub fn set_authority(
        ctx: Context<AdminInstruction>,
        role: AuthRole,
        operators: Vec<Pubkey>,
    ) -> Result<()> {
        admin_instruction::set_authority_handler(ctx, role, operators)
    }

    // pub fn set_status(ctx: Context<AdminInstruction>, status: DepositStatus) -> Result<()> {
    //     admin_instruction::set_status_handler(ctx, &status)
    // }

    pub fn deposit(ctx: Context<UserDeposit>, amount: u64) -> Result<()> {
        user_deposit::user_deposit_handle(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw_handle(ctx, amount)
    }
}
