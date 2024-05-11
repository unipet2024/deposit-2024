use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod types;
pub mod utils;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use types::*;
pub use utils::*;


declare_id!("7G5ruteouLxPrkqRo4pe8DCMw78JYo5bsnV7jyLkUFKx");

#[program]
pub mod deposit_2024 {
    use super::*;

    pub fn init(ctx: Context<InitDeposit>,  operator_wallet: Pubkey) -> Result<()> {
        init_deposit::init_handle(ctx,  operator_wallet)
    }

    pub fn create_packages(
        ctx: Context<OperatorCreatePackage>,
        data: PackageInitParams,
    ) -> Result<()> {
        operator_instruction::handle_create_package(ctx, data)
    }

    pub fn set_admin_authority(
        ctx: Context<SetAdminInstruction>,
        operators: Pubkey,
    ) -> Result<()> {
        set_admin::handle_set_admin(ctx, operators)
    }

    // pub fn set_status(ctx: Context<AdminInstruction>, status: DepositStatus) -> Result<()> {
    //     admin_instruction::set_status_handler(ctx, &status)
    // }

    pub fn buy_package(ctx: Context<UserBuyPackage>, package_id: u16) -> Result<()> {
        user_buy_package_spl::handle_user_buy_package(ctx, package_id )
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw_handle(ctx, amount)
    }
}
