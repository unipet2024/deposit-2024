use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};

use crate::{ Deposit, DepositErrors, DepositEvent, Package, User, DEPOSIT_ACCOUNT, PACKAGE, USER_ACCOUNT};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct UserDeposit<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump,
        // constraint = deposit.status == DepositStatus::Open @ DepositErrors::DepositClosed,
    )]
    pub deposit: Box<Account<'info, Deposit>>,

    #[account(
        init_if_needed,
        space =   8+20,
        payer = user,
        seeds = [USER_ACCOUNT, user.key().as_ref()],
        // seeds = [USER_ACCOUNT],
        bump,
    )]
    pub user_deposit: Account<'info, User>,

    #[account(
        seeds = [PACKAGE, token_mint.key().as_ref()],
        bump,
        constraint = package_account.valid == true
    )]
    pub package_account:  Account<'info, Package>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        associated_token::mint = token_mint, 
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = deposit,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn user_deposit_handle(ctx: Context<UserDeposit>, id: u64) -> Result<()> {
    // let deposit = &ctx.accounts.deposit;
    let user_deposit = &mut ctx.accounts.user_deposit;
    let token_mint = &mut ctx.accounts.token_mint;
    let user_ata = &mut ctx.accounts.user_ata;
    let deposit_ata = &mut ctx.accounts.deposit_ata;
    let user = &mut ctx.accounts.user;
    let package_account = &ctx.accounts.package_account;

    // let vault_ata = get_associated_token_address(&deposit.vault, &token_mint.key());

    let package_item = package_account.get_package(id);
    msg!("Amount: {:?}", package_item);


    require_neq!(package_item.price, 0, DepositErrors::InputInvalid);

    // require_eq!(
    //     deposit.check_token_support(&token_mint.key()),
    //     true,
    //     DepositErrors::TokenNotSupport
    // );

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
        package_item.price,
    )?;

    msg!("Update user info");
    let clock = Clock::get()?;

    // user_deposit.amount += package_item.price;
    emit!(DepositEvent {
        token: token_mint.key(),
        user: user.key(),
        package_item,
        time: clock.unix_timestamp
    });

    Ok(())
}
