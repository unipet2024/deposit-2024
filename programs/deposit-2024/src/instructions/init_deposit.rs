use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, ADMIN_ROLE, DEPOSIT_ACCOUNT};


#[derive(Accounts)]
pub struct InitDeposit<'info> {
    #[account(
        init,  
        payer = authority, 
        space = 8 + 230,
        seeds = [DEPOSIT_ACCOUNT],
        bump
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE, authority.key().as_ref()], 
        bump,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn init_handle(ctx: Context<InitDeposit>, operator_wallet: Pubkey) -> Result<()> {
    let deposit = &mut ctx.accounts.deposit_account;
    let admin_account = &mut ctx.accounts.admin_account;
    let authority = &ctx.accounts.authority;


    deposit.init( authority.key(), operator_wallet, ctx.bumps.deposit_account)?;

    admin_account.initialize(
        &authority.key(),
        ctx.bumps.admin_account,
        AuthRole::Admin,
    )?;

    Ok(())
}
