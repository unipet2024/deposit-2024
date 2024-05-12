use anchor_lang::prelude::*;

use crate::{ DEPOSIT_ACCOUNT, OPERATOR_ROLE, PACKAGE};
use crate::{Deposit, AuthRole, AuthorityRole, PackageInitParams, DepositErrors, Package,};

use anchor_lang::prelude::Signer;

#[derive(Accounts)]
#[instruction(param: PackageInitParams)]

pub struct CreatePackage<'info> {
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    #[account(
        init_if_needed,  
        space = 8 + 16, 
        payer = operator,
        seeds = [PACKAGE, param.id.to_le_bytes().as_ref()],
        bump,
    )]
    pub package_account:  Account<'info, Package>,

    #[account(
        seeds = [OPERATOR_ROLE, operator.key().as_ref()], 
        bump = operator_account.bump,
        constraint = operator_account.is_authority(operator.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>, 
}








pub fn handle_create_package(ctx: Context<CreatePackage>, data: PackageInitParams) -> Result<()> {
    let package_account = &mut ctx.accounts.package_account;
    let deposit_account = &mut ctx.accounts.deposit_account;
    
    deposit_account.add_package(data.id)?;
    package_account.init(data.id, data.price, ctx.bumps.package_account)?;

    Ok(())
}

