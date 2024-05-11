use std::ops::Mul;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};

use crate::{ Deposit, DepositErrors, DepositEvent, Package, User, DEPOSIT_ACCOUNT, PACKAGE, USER_ACCOUNT};

#[derive(Accounts)]
#[instruction(package_id: u16)]
pub struct UserBuyPackage<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
        constraint = deposit_account.is_support_currency(token_mint.key()) @ DepositErrors::CurrencyNotSupport,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        seeds = [USER_ACCOUNT, user.key().as_ref()],
        bump,
    )]
    pub user_deposit: Account<'info, User>,

    #[account(
        seeds = [PACKAGE, package_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub package_account:  Account<'info, Package>,


    #[account(
        mut,
        associated_token::mint = token_mint, 
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = deposit_account,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub user: Signer<'info>, 
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handle_user_buy_package(ctx: Context<UserBuyPackage>, package_id: u16) -> Result<()> {
    // let deposit = &ctx.accounts.deposit;
    let user_deposit = &mut ctx.accounts.user_deposit;
    let token_mint = &ctx.accounts.token_mint;

    let user = &mut ctx.accounts.user;
    let package_account = &ctx.accounts.package_account;



    let price  = package_account.price as u64;
    let decimal = token_mint.decimals ;

    let amount = price.mul(10u64.pow(decimal as u32));

    let user_ata = &mut ctx.accounts.user_ata;
    let deposit_ata = &mut ctx.accounts.deposit_ata;
    msg!("Transfer from User to Deposit");
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: user.to_account_info(),
                from: user_ata.to_account_info(),
                to: deposit_ata.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("update user deposit account");
    user_deposit.add_package_bought(package_id)?;

    //emit event
    let clock = Clock::get()?;

    // user_deposit.amount += package_item.price;
    emit!(DepositEvent {
        package: package_id,
        user: user.key(),
        time: clock.unix_timestamp
    });

    Ok(())
}
