use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, DepositErrors, SetAuthorityEvent, ADMIN_ROLE, DEPOSIT_ACCOUNT};

#[derive(Accounts)]
#[instruction(new_admin: Pubkey)]
pub struct SetAdminInstruction<'info> {
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
        constraint = deposit_account.is_admin(authority.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE, new_admin.as_ref()], 
        bump,
    )]
    pub new_admin_account:  Account<'info, AuthorityRole>,

    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.is_authority(authority.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn handle_set_admin(ctx: Context<SetAdminInstruction>, new_admin: Pubkey) -> Result<()> {
    let deposit = &mut ctx.accounts.deposit_account;
    let new_admin_account = &mut ctx.accounts.new_admin_account;
    deposit.set_admin(new_admin)?;
    new_admin_account.initialize(&new_admin, ctx.bumps.new_admin_account, AuthRole::Admin)?;

    emit!(SetAuthorityEvent{
        admin: new_admin,
        role: AuthRole::Admin,
        time: Clock::get()?.unix_timestamp
    });
    Ok(())
}


#[derive(Accounts)]
pub struct CloseAdminInstruction<'info> {
    #[account(mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
        constraint = deposit_account.is_admin(payer.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        mut, close = payer
    )]
    pub close_admin_account:  Account<'info, AuthorityRole>,

    #[account(
        mut,
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


pub fn handle_close_admin(ctx: Context<CloseAdminInstruction>, _admin: Pubkey) -> Result<()> {
    let deposit = &mut ctx.accounts.deposit_account;
    deposit.remove_admin(_admin)?;
    Ok(())
}






