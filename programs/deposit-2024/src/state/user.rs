use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub bump: u8,    //1
    pub owner: Pubkey, //32
    pub bought_package:  Vec<u16>, //4 + n * 8 

}

impl User {
    pub fn initialize(&mut self, owner: &Pubkey, bump: u8) -> Result<()> {
        self.owner = *owner;
        self.bump = bump;
        self.bought_package = vec![];
        Ok(())
    }
    pub fn add_package_bought(&mut self, package_id: u16) -> Result<()> {
        self.bought_package.push(package_id);
        Ok(())
    }

    pub fn get_counter_package(&self) -> usize {
        self.bought_package.len()
    }
}
