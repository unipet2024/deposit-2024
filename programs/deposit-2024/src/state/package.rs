use anchor_lang::prelude::*;

use crate::DepositErrors;



// total 230
#[account]
pub struct Package {
    pub bump: u8,  
    pub status: bool,   
    pub id: u16, 
    pub price: u32 ,     
       
}

impl Package {

    pub fn init(&mut self, id: u16, price: u32, bump: u8) -> Result<()> {
        self.id = id;
        self.price = price;
        self.bump = bump;
        self.status = true;
        Ok(())
    }  
    pub fn update_price(&mut self, id: u16, price: u32) -> Result<()> {
        if self.id != id {
            return Err(DepositErrors::InputInvalid.into());
        }
        self.price = price;
        Ok(())
    }
    pub fn close(&mut self) -> Result<()> {
        self.status = false;
        Ok(())
    }
}


#[account]
pub struct ResultAccount {
    pub value: i128,
}
