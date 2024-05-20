use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{ AuthRole, AuthorityRole, Deposit, DepositErrors, TokenTransferParams, WithdrawEvent,_transfer_token, DEPOSIT_ACCOUNT, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct OperatorWithdrawSpl<'info>{
    #[account( 
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
        constraint = deposit_account.operator_wallet == operator_wallet.key(),
        constraint = deposit_account.is_support_currency(token_mint.key()) @ DepositErrors::TokenNotSupport,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref() ], 
        bump = operator_account.bump,
        constraint = operator_account.is_authority(authority.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(
        constraint = operator_wallet.key() == deposit_account.operator_wallet,
    )]
     /// CHECK: Create a new associated token account for the operator account
     pub operator_wallet: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint, 
        associated_token::authority = operator_wallet,
    )]
    pub operator_wallet_ata: Account<'info, TokenAccount>,



    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = deposit_account,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>, 

}


pub fn withdraw_spl_handle(ctx: Context<OperatorWithdrawSpl>, amount: u64) -> Result<()> {
    let deposit_pad = &mut ctx.accounts.deposit_account;
    let token_mint = &ctx.accounts.token_mint;
   
    let to_ata = &mut ctx.accounts.operator_wallet_ata;
    let from_ata = &mut ctx.accounts.deposit_ata;

    msg!("Tranfer {:} {:} to admin", amount, token_mint.key());
    require!(from_ata.amount >= amount, DepositErrors::InsufficientAmount);

    let seeds: &[&[u8]] = &[DEPOSIT_ACCOUNT, &[deposit_pad.bump]];

    _transfer_token(
        &TokenTransferParams {
            source: from_ata.to_account_info(),
            destination: to_ata.to_account_info(),
            authority: deposit_pad.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority_signer_seeds: seeds,
            amount,
        }
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct WithdrawSol<'info>{
    #[account( 
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    
    #[account(
        seeds = [OPERATOR_ROLE, authority.key().as_ref()], 
        bump = operator_account.bump,
        constraint = operator_account.is_authority(authority.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,


     /// CHECK: Create a new associated token account for the operator account
    #[account(
        mut,
        constraint = deposit_account.operator_wallet == operator_wallet.key(),
    )]
     pub operator_wallet: AccountInfo<'info>,

    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

pub fn withdraw_sol_handle(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {

    require!(amount > 0, DepositErrors::InvalidAmount);
    let deposit_pad = &mut ctx.accounts.deposit_account;
    let operator_wallet =&mut ctx.accounts.operator_wallet;

    let rent_balance = Rent::get()?.minimum_balance(deposit_pad.to_account_info().data_len());
    let max_withdraw_amount = **deposit_pad.to_account_info().lamports.borrow() - rent_balance;

    require!(amount <= max_withdraw_amount, DepositErrors::InsufficientAmount);
    **deposit_pad.to_account_info().try_borrow_mut_lamports()? -= amount;
    **operator_wallet.try_borrow_mut_lamports()? += amount;

    // let ix = anchor_lang::solana_program::system_instruction::transfer(
    //     &deposit_pad.key(),
    //     &operator_wallet.key(),
    //     amount);

    // let seeds: &[&[u8]] = &[DEPOSIT_ACCOUNT, &[deposit_pad.bump]];
    // let signer:[&[&[u8]]; 1] = [&seeds[..]];
    
    // anchor_lang::solana_program::program::invoke_signed(
    //     &ix,
    //     &[
    //         deposit_pad.to_account_info(),
    //         operator_wallet.to_account_info(),
    //     ],
    //     &signer,
    // )?;
   //emit event 
    emit!(WithdrawEvent{
        address: operator_wallet.key(),
        amount: amount,
        time: Clock::get().unwrap().unix_timestamp
    });

    Ok(())
}


