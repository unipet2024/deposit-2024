use anchor_lang::prelude::*;

use crate::{AuthRole, AuthorityRole, Deposit, ADMIN_ROLE, DEPOSIT_ACCOUNT, OPERATOR_ROLE};


#[derive(Accounts)]
pub struct InitDeposit<'info> {
    #[account(
        init_if_needed,  
        payer = authority, 
        space =8+230,
        seeds = [DEPOSIT_ACCOUNT],
        bump
    )]
    pub deposit: Box<Account<'info, Deposit>>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [ADMIN_ROLE], 
        bump,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(
        init_if_needed,
        space = 60,
        payer = authority,
        seeds = [OPERATOR_ROLE], 
        bump,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn init_handle(ctx: Context<InitDeposit>) -> Result<()> {
    let deposit = &mut ctx.accounts.deposit;
    let admin_account = &mut ctx.accounts.admin_account;
    let operator_account = &mut ctx.accounts.operator_account;


    deposit.init(
        admin_account.key(),
        operator_account.key(),
        // vault,
        ctx.bumps.deposit,
        // &tokens.token,
    )?;

    //SET ADMIN
   //SET ADMIN
   let authorities = vec![ctx.accounts.authority.key()];
   admin_account.initialize(
       &authorities,
       ctx.bumps.admin_account,
       AuthRole::Admin,
   )?;
   operator_account.initialize(
       &authorities,
       ctx.bumps.operator_account,
       AuthRole::Operator,
   )?;

    Ok(())
}
