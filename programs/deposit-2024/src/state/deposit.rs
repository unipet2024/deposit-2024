use anchor_lang::prelude::*;

// use crate::DepositStatus;

// total 230
#[account]
pub struct Deposit {
    pub admin: Pubkey,    //32
    pub operator: Pubkey, //32
    // pub vault: Pubkey,           //32
    // pub tokens: Vec<Pubkey>,   // Max 5 => 4+ 32*5=164
    // pub status: DepositStatus, //1
    pub bump: u8,              //1
}

impl Deposit {
    pub fn init(
        &mut self,
        admin: Pubkey,
        operator: Pubkey,
        // vault: Pubkey,
        bump: u8, // tokens: &Vec<Pubkey>,
    ) -> Result<()> {
        self.admin = admin;
        self.operator = operator;
        // self.vault = vault;
        self.bump = bump;
        // self.status = DepositStatus::Waiting;

        // self.set_tokens(&tokens)?;

        Ok(())
    }

    // pub fn set_tokens(&mut self, tokens: &Vec<Pubkey>) -> Result<()> {
    //     self.tokens = vec![];
    //     for (_, token) in tokens.iter().enumerate() {
    //         self.add_whilelist(*token);
    //     }
    //     Ok(())
    // }

    // fn add_whilelist(&mut self, token: Pubkey) {
    //     self.tokens.push(token)
    // }

    // pub fn check_token_support(&mut self, token: &Pubkey) -> bool {
    //     if self.tokens.contains(token) {
    //         return true;
    //     } else {
    //         return false;
    //     }
    // }

    // pub fn set_status(&mut self, status: DepositStatus) {
    //     self.status = status;
    // }
}
