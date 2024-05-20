

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use chainlink_solana as chainlink;

use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};
use solana_program::native_token::LAMPORTS_PER_SOL;
use solana_safe_math::SafeMath;

use crate::{ Deposit, DepositErrors, DepositEvent, Package, User, DEPOSIT_ACCOUNT, PACKAGE, USER_ACCOUNT};

#[derive(Accounts)]
#[instruction(package_id: u16)]
pub struct UserBuyPackageBySpl<'info> {
    #[account(
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
        constraint = deposit_account.is_support_currency(token_mint.key()) @ DepositErrors::CurrencyNotSupport,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 1000,
        seeds = [USER_ACCOUNT, user.key().as_ref()],
        bump,
    )]
    pub user_deposit: Account<'info, User>,

    #[account(
        seeds = [PACKAGE, package_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub package_account:  Account<'info, Package>,

    #[account(mut,
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

#[derive(Accounts)]
#[instruction(package_id: u16)]
pub struct UserBuyPackageBySol<'info> {
    #[account(
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump = deposit_account.bump,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + 1000,
        seeds = [USER_ACCOUNT, user.key().as_ref()],
        bump,
    )]
    pub user_deposit: Account<'info, User>,

    #[account(
        seeds = [PACKAGE, package_id.to_le_bytes().as_ref()],
        bump = package_account.bump,
    )]
    pub package_account:  Account<'info, Package>,
    /// CHECK: get price from chainlink
    pub chainlink_program: AccountInfo<'info>,
    /// CHECK: get price from chainlink
    pub chainlink_feed: AccountInfo<'info>,
    #[account(mut, signer)]
    pub user: Signer<'info>, 
    pub system_program: Program<'info, System>, 
}


pub fn handle_user_buy_package_by_spl(ctx: Context<UserBuyPackageBySpl>, package_id: u16) -> Result<()> {
    // let deposit = &ctx.accounts.deposit;
    let user_deposit = &mut ctx.accounts.user_deposit;
    let token_mint = &ctx.accounts.token_mint;

    let user = &mut ctx.accounts.user;
    let package_account = &ctx.accounts.package_account;



    let price  = package_account.price as u64;
    let decimal = token_mint.decimals ;

    let amount = price.safe_mul(10u64.pow(decimal as u32))?.safe_div(10000)?; // price in 4 decimals

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
       //update user deposit account
       if user_deposit.bump == 0 {
        user_deposit.initialize(user.key, ctx.bumps.user_deposit)?;
    }else{
        user_deposit.add_package_bought(package_id)?;
    }

    //emit event
    let clock = Clock::get().unwrap();
    emit!(DepositEvent {
        package: package_id,
        user: user.key(),
        time:clock.unix_timestamp,
        amount: amount,
        token: token_mint.key().to_string(),
        slot: clock.slot,
    });

    Ok(())
}


pub fn handle_user_buy_package_by_sol(ctx: Context<UserBuyPackageBySol>, package_id: u16) -> Result<()> {
    let deposit = &ctx.accounts.deposit_account;
    let user_deposit = &mut ctx.accounts.user_deposit;
    let package_account = &ctx.accounts.package_account;

    let user = &mut ctx.accounts.user;

    let package_price  = package_account.price as u64;
    msg!("package_price: {}", package_price);

    let round = chainlink::latest_round_data(ctx.accounts.chainlink_program.to_account_info(), ctx.accounts.chainlink_feed.to_account_info())?;

    
    let sol_price = round.answer ; 
    msg!("sol_price: {}", sol_price );

    //package price in 4 decimals, sol price in 8 decimals,  1 sol = 10^9 ,
    let sol_amount  = package_price.safe_mul(100000000)?.safe_div(sol_price as u64)?.safe_mul(LAMPORTS_PER_SOL)?.safe_div(10000)?;
    msg!("sol_amount: {}", sol_amount);

    
    //get user lam port
    let user_lamport = user.get_lamports();
    //check balance

    require!(user_lamport >= sol_amount , DepositErrors::InsufficientAmount);


    //transfer sol to deposit
    let instruction = anchor_lang::solana_program::system_instruction::transfer(
        user.key,
        &deposit.key(),
        sol_amount,
    );


    anchor_lang::solana_program::program::invoke(
        &instruction,
        &[user.to_account_info(), deposit.to_account_info()],
    )?;

    //update user deposit account
    if user_deposit.bump == 0 {
        user_deposit.initialize(user.key, ctx.bumps.user_deposit)?;
    }else{
        user_deposit.add_package_bought(package_id)?;
    }
    //emit event

    let clock = Clock::get().unwrap();
    emit!(DepositEvent {
        package: package_id,
        user: user.key(),
        time: clock.unix_timestamp,
        amount: sol_amount,
        token: Pubkey::default().to_string(),
        slot: clock.slot,
    });

    Ok(())
}
