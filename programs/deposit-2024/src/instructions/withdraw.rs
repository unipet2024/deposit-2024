use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::{ AuthRole, AuthorityRole, Deposit, DepositErrors, DEPOSIT_ACCOUNT};
use anchor_spl::{
    // associated_token::get_associated_token_address,
    token::{transfer, Transfer as SplTransfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account( 
        seeds = [DEPOSIT_ACCOUNT],
        bump,
        constraint = deposit.admin == admin_account.key() @ DepositErrors::OnlyAdmin,
    )]
    pub deposit: Box<Account<'info, Deposit>>,

    #[account( 
        constraint = admin_account.is_authority(admin.key) == true @ DepositErrors::OnlyAdmin, 
        constraint = admin_account.role == AuthRole::Admin @ DepositErrors::OnlyAdmin,
        constraint = admin_account.status == true @ DepositErrors::OnlyAdmin,
    )]
    pub admin_account:  Account<'info, AuthorityRole>,

    pub token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer=admin,
        associated_token::mint = token_mint, 
        associated_token::authority = admin
    )]
    pub admin_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = deposit,
    )]
    pub deposit_ata: Account<'info, TokenAccount>,

    #[account(mut, signer)]
    pub admin: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>, 
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn withdraw_handle(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let deposit = &mut ctx.accounts.deposit;
    let admin_ata = &mut ctx.accounts.admin_ata;
    let token_mint = &mut ctx.accounts.token_mint;
    let deposit_ata = &mut ctx.accounts.deposit_ata;

    msg!("Tranfer {:} {:} to admin", amount, token_mint.key());

    let seeds: &[&[u8]] = &[DEPOSIT_ACCOUNT, &[deposit.bump]];
    let signer = &[&seeds[..]];

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                authority: deposit.to_account_info(),
                from: deposit_ata.to_account_info(),
                to: admin_ata.to_account_info(),
            },
        )
        .with_signer(signer),
        amount,
    )?;

    Ok(())
}