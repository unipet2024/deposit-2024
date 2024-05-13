use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{ AuthRole, AuthorityRole, Deposit, DepositErrors, TokenTransferParams, WithdrawEvent,_transfer_token, DEPOSIT_ACCOUNT, ADMIN_ROLE};




#[derive(Accounts)]
pub struct AdminWithdrawSol<'info>{
    #[account( 
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
        constraint = deposit_account.is_admin(authority.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
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
}

pub fn admin_withdraw_sol_handle(ctx:Context<AdminWithdrawSol>)-> Result<()>{
    let deposit_pad = &mut ctx.accounts.deposit_account;
    let authority = &mut ctx.accounts.authority;

    let rent_balance = Rent::get()?.minimum_balance(deposit_pad.to_account_info().data_len());
    let withdraw_amount = **deposit_pad.to_account_info().lamports.borrow() - rent_balance;
   
    require!(withdraw_amount > 0, DepositErrors::InsufficientAmount);

    **deposit_pad.to_account_info().try_borrow_mut_lamports()? -= withdraw_amount;
    **authority.to_account_info().try_borrow_mut_lamports()? += withdraw_amount;
    Ok(())
}



#[derive(Accounts)]
pub struct AdminWithdrawSpl<'info>{
    #[account( 
        mut,
        seeds = [DEPOSIT_ACCOUNT],
        bump =  deposit_account.bump,
        constraint = deposit_account.is_admin(authority.key()) @ DepositErrors::AdminAccountInvalid,
    )]
    pub deposit_account: Box<Account<'info, Deposit>>,
    #[account(
        seeds = [ADMIN_ROLE, authority.key().as_ref()],
        bump = admin_account.bump,
        constraint = admin_account.is_authority(authority.key) == true @ DepositErrors::OnlyAdmin,
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = deposit_account,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = authority,
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    pub token_mint: Account<'info, Mint>,
    #[account(mut, signer)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
}


pub fn admin_withdraw_spl_handle(ctx:Context<AdminWithdrawSpl>)-> Result<()>{
    let deposit_pad = &ctx.accounts.deposit_account;
    let authority = &mut ctx.accounts.authority;
    let deposit_ata =  &mut ctx.accounts.deposit_ata; 
    let admin_ata =  &mut ctx.accounts.admin_ata; 

    let amount   = deposit_ata.amount;
    require!(amount > 0, DepositErrors::InvalidAmount);

    let seeds: &[&[u8]] = &[DEPOSIT_ACCOUNT, &[deposit_pad.bump]];
    _transfer_token(
        &TokenTransferParams {
            source: deposit_ata.to_account_info(),
            destination: admin_ata.to_account_info(),
            authority: deposit_pad.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            authority_signer_seeds: seeds,
            amount,
        }
    )?;
    //emit event
    emit!(WithdrawEvent{
        address: authority.key(),
        amount,
        time:Clock::get()?.unix_timestamp
    });

    Ok(())
}