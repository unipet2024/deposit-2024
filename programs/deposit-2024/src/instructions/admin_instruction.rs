use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, ChangeOperatorWalletEvent, Deposit, DepositErrors, ADMIN_ROLE, DEPOSIT_ACCOUNT};

#[derive(Accounts)]
pub struct AdminSetupInstruction<'info> {
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump= deposit_account.bump,
        constraint = deposit_account.is_admin(admin.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        seeds = [ADMIN_ROLE, admin.key().as_ref()], 
        bump = admin_account.bump,
        constraint = admin_account.is_authority(admin.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}


pub fn handle_change_operator_wallet(ctx: Context<AdminSetupInstruction>, operators: Pubkey) -> Result<()> {

    let deposit = &mut ctx.accounts.deposit_account;
    deposit.set_operator(operators)?;

    emit!(ChangeOperatorWalletEvent{
        wallet: operators,
        time:Clock::get()?.unix_timestamp
    });
    Ok(())
}

pub fn handle_add_currency(ctx: Context<AdminSetupInstruction>, currency: Pubkey) -> Result<()> {

    let deposit = &mut ctx.accounts.deposit_account;
    deposit.add_currency(currency)?;

    emit!(ChangeOperatorWalletEvent{
        wallet: currency,
        time:Clock::get()?.unix_timestamp
    });
    Ok(())
}

pub fn handle_remove_currency(ctx: Context<AdminSetupInstruction>, currency: Pubkey) -> Result<()> {

    let deposit = &mut ctx.accounts.deposit_account;
    deposit.remove_currency(currency)?;

    emit!(ChangeOperatorWalletEvent{
        wallet: currency,
        time:Clock::get()?.unix_timestamp
    });
    Ok(())
}