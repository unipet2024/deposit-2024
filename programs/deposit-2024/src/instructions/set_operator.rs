use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, DepositErrors, SetAuthorityEvent, ADMIN_ROLE, OPERATOR_ROLE, DEPOSIT_ACCOUNT};

#[derive(Accounts)]
#[instruction(operator: Pubkey)]
pub struct SetOperatorInstruction<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
        constraint = deposit_account.is_admin(payer.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        init_if_needed,
        space = 60,
        payer = payer,
        seeds = [OPERATOR_ROLE, operator.as_ref()], 
        bump,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(
        seeds = [ADMIN_ROLE, payer.key().as_ref()], 
        bump = admin_account.bump,
        constraint = admin_account.is_authority(payer.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>, 
}


#[derive(Accounts)]
pub struct CloseOperatorInstruction<'info> {

    #[account(
        mut, close = payer
    )]
    pub close_operator_account:  Account<'info, AuthorityRole>,

    #[account(
        seeds = [ADMIN_ROLE, payer.key().as_ref()], 
        bump = admin_account.bump,
        constraint = admin_account.is_authority(payer.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>, 
}







pub fn handle_set_operator(ctx: Context<SetOperatorInstruction>, new_operator: Pubkey) -> Result<()> {
    let operator_account = &mut ctx.accounts.operator_account;
    
    operator_account.initialize(&new_operator, ctx.bumps.operator_account, AuthRole::Operator)?;

    emit!(SetAuthorityEvent{
        admin: new_operator,
        role: AuthRole::Operator,
        time: Clock::get()?.unix_timestamp
    });
    Ok(())
}

pub fn handle_close_operator(_: Context<CloseOperatorInstruction>, _: Pubkey) -> Result<()> {

    Ok(())
}




