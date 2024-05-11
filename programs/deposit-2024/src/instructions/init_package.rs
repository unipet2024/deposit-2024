use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, DepositErrors, Package, PackageItem, SetPackageEvent, DEPOSIT_ACCOUNT, OPERATOR_ROLE, PACKAGE};

#[derive(Accounts)]
#[instruction(token: Pubkey, packages: Vec<PackageItem>, valid: bool)]
pub struct InitPackage<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump,
        constraint = deposit.operator == operator_account.key() @ DepositErrors::OperatorAccountInvalid,
    )]
    pub deposit: Box<Account<'info, Deposit>>,

    #[account(
        seeds = [OPERATOR_ROLE], 
        bump,
        constraint = operator_account.is_authority(operator.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(
        init_if_needed,  
        space = 8 + 6 + packages.len() * 24, 
        payer = operator,
        seeds = [PACKAGE, token.as_ref()],
        bump,
    )]
    pub package_account:  Account<'info, Package>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn set_package_handle(ctx: Context<InitPackage>,packages: PackageItem) -> Result<()> {
    
    let package_account = &mut ctx.accounts.package_account;
    // require_eq!(package_account.valid, true, DepositErrors::PackageInvalid);

    package_account.valid= true;
    package_account.set_packages(&packages)?;

    emit!(SetPackageEvent{
        operator: ctx.accounts.operator.key(),
        token,
        packages,
        time: Clock::get()?.unix_timestamp
    });
    Ok(())
}

