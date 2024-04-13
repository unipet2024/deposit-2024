use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, DepositErrors, SetAuthorityEvent, ADMIN_ROLE, DEPOSIT_ACCOUNT, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct AdminInstruction<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump=deposit.bump,
        constraint = deposit.admin == admin_account.key() @ DepositErrors::AdminAccountInvalid,
        constraint = deposit.operator == operator_account.key() @ DepositErrors::OperatorAccountInvalid,
    )]
    pub deposit: Box<Account<'info, Deposit>>,

    #[account(
        mut,
        seeds = [ADMIN_ROLE], 
        bump=admin_account.bump,
        constraint = admin_account.is_authority(admin.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(
        mut,
        seeds = [OPERATOR_ROLE], 
        bump=operator_account.bump,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn set_authority_handler(ctx: Context<AdminInstruction>, role: AuthRole, operators: Vec<Pubkey>) -> Result<()> {
    match role {
        AuthRole::Operator => set_operator_handler(ctx, operators),
        AuthRole::Admin => set_admin_handler(ctx, operators),
    }
}

fn set_operator_handler(ctx: Context<AdminInstruction>, operators: Vec<Pubkey>) -> Result<()> {
    let operator_account = &mut ctx.accounts.operator_account;

    for operator in operators.iter(){
        msg!("{:},", *operator)
    }

    operator_account.set_authorities(&operators)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Operator,
        operators,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}

fn set_admin_handler(ctx: Context<AdminInstruction>, admins: Vec<Pubkey>) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_account;

    admin_account.set_authorities(&admins)?;

    emit!(SetAuthorityEvent{
        admin: ctx.accounts.admin.key(),
        role: AuthRole::Admin,
        operators: admins,
        time: Clock::get()?.unix_timestamp
    });

    Ok(())
}

// pub fn set_status_handler(ctx: Context<AdminInstruction>, status: &DepositStatus) -> Result<()> {
//     let deposit = &mut ctx.accounts.deposit;

//     deposit.set_status(*status);

//     emit!(SetStatusEvent{
//         admin: ctx.accounts.admin.key(),
//         status: *status,
//         time: Clock::get()?.unix_timestamp
//     });

//     Ok(())
// }