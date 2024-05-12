use anchor_lang::prelude::*;



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
}


#[account]
pub struct ResultAccount {
    pub value: i128,
}
