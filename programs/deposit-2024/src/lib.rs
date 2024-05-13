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
    pub fn add_currency(ctx: Context<AdminSetupInstruction>, currency: Pubkey) -> Result<()> {
        admin_instruction::handle_add_currency(ctx, currency)
    }

    pub fn add_operator(ctx: Context<SetOperatorInstruction>, new_operator: Pubkey) -> Result<()> {
        set_operator::handle_set_operator(ctx, new_operator)
    }
    pub fn create_packages(
        ctx: Context<CreatePackage>,
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

    pub fn user_buy_package_by_spl(ctx: Context<UserBuyPackageBySpl>, package_id: u16) -> Result<()> {
        user_buy_package::handle_user_buy_package_by_spl(ctx, package_id )
    }

    pub fn user_buy_package_by_sol(ctx: Context<UserBuyPackageBySol>, package_id: u16) -> Result<()> {
        user_buy_package::handle_user_buy_package_by_sol(ctx, package_id )
    }

    pub fn admin_withdraw_spl(ctx: Context<AdminWithdrawSpl>) -> Result<()> {
        admin_withdraw::admin_withdraw_spl_handle(ctx)
    }

    pub fn admin_withdraw_sol(ctx: Context<AdminWithdrawSol>) -> Result<()> {
        admin_withdraw::admin_withdraw_sol_handle(ctx)
    }
}
