use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{ AuthRole, AuthorityRole, Deposit, DepositErrors, TokenTransferParams, _transfer_token, DEPOSIT_ACCOUNT, OPERATOR_ROLE};

#[derive(Accounts)]
pub struct WithdrawSpl<'info>{
    #[account( 
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
        constraint = deposit_account.is_support_currency(token_mint.key()) @ DepositErrors::TokenNotSupport,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    
    #[account(
        seeds = [OPERATOR_ROLE], 
        bump = operator_account.bump,
        constraint = operator_account.is_authority(operator.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = operator,
        associated_token::mint = token_mint, 
        associated_token::authority = operator_wallet,
    )]
    pub operator_wallet_ata: Account<'info, TokenAccount>,

     /// CHECK: Create a new associated token account for the operator account
    pub operator_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = deposit_account,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn withdraw_spl_handle(ctx: Context<WithdrawSpl>, amount: u64) -> Result<()> {
    let deposit_pad = &mut ctx.accounts.deposit_account;
    let to_ata = &mut ctx.accounts.operator_wallet_ata;
    let token_mint = & ctx.accounts.token_mint;
    let from_ata = &mut ctx.accounts.deposit_ata;
    let token_program = &ctx.accounts.token_program;

    msg!("Tranfer {:} {:} to admin", amount, token_mint.key());
    require!(from_ata.amount >= amount, DepositErrors::InsufficientAmount);

    let seeds: &[&[u8]] = &[DEPOSIT_ACCOUNT, &[deposit_pad.bump]];

    _transfer_token(
        &TokenTransferParams {
            source: from_ata.to_account_info(),
            destination: to_ata.to_account_info(),
            authority: deposit_pad.to_account_info(),
            token_program: token_program.to_account_info(),
            authority_signer_seeds: seeds,
            amount,
        }
    )?;

    Ok(())
}


#[derive(Accounts)]
pub struct WithdrawSol<'info>{
    #[account( 
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    
    #[account(
        seeds = [OPERATOR_ROLE, operator.key().as_ref()], 
        bump = operator_account.bump,
        constraint = operator_account.is_authority(operator.key) == true @ DepositErrors::OnlyOperator,
        constraint = operator_account.role == AuthRole::Operator @ DepositErrors::OnlyOperator,
        constraint = operator_account.status == true @ DepositErrors::OnlyOperator,
    )]
    pub operator_account:  Account<'info, AuthorityRole>,

    #[account(mut)]
     /// CHECK: Create a new associated token account for the operator account
     pub operator_wallet: UncheckedAccount<'info>,

    #[account(mut, signer)]
    pub operator: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
}

pub fn withdraw_sol_handle(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    let deposit_pad = &mut ctx.accounts.deposit_account;
    let operator_wallet =&mut ctx.accounts.operator_wallet;

    let rent_balance = Rent::get()?.minimum_balance(deposit_pad.to_account_info().data_len());
    let withdraw_amount = **deposit_pad.to_account_info().lamports.borrow() - rent_balance;

    require!( withdraw_amount >= amount, DepositErrors::InsufficientAmount);

    **deposit_pad.to_account_info().try_borrow_mut_lamports()? -= amount;
    **operator_wallet.to_account_info().try_borrow_mut_lamports()? += amount;

    Ok(())
}