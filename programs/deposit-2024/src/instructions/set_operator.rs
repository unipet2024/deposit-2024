use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, DepositErrors, SetAuthorityEvent, ADMIN_ROLE, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct SetOperatorInstruction<'info> {

    #[account(
        init_if_needed,
        space = 60,
        payer = payer,
        seeds = [OPERATOR_ROLE], 
        bump,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(
        seeds = [ADMIN_ROLE], 
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


#[derive(Accounts)]
pub struct CloseOperatorInstruction<'info> {

    #[account(
        mut, close = payer
    )]
    pub close_operator_account:  Account<'info, AuthorityRole>,

    #[account(
        seeds = [ADMIN_ROLE], 
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




