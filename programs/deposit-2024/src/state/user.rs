use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub bought_package:  Vec<u32>, //4 + n * 8 

}

impl User {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        self.bump = bump;
        self.bought_package = vec![];
        Ok(())
    }
}
