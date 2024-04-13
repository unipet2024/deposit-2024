use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,    //1
    pub amount: u64, //8
}

impl User {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        self.amount = 0;
        Ok(())
    }
}
