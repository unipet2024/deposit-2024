
use anchor_lang::prelude::*;


use crate::TokenTransferParams;

#[inline(always)]
pub fn _transfer_token<'a>(params: &'a TokenTransferParams) -> Result<()> {
    let transfer_instruction = anchor_spl::token::Transfer {
        from: params.source.to_account_info(),
        to: params.destination.to_account_info(),
        authority: params.authority.to_account_info(),
    };
    let cpi_program = params.token_program.to_account_info();
    let singer: [&[&[u8]]; 1] = [params.authority_signer_seeds];
    let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction).with_signer(&singer);
    anchor_spl::token::transfer(cpi_ctx, params.amount)?;
    Ok(())
}
