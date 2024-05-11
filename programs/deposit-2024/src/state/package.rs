use anchor_lang::prelude::*;



// total 230
#[account]
pub struct Package {
    pub id: u16,                    //8
    pub status: bool,
    pub price: u16 ,             //1
    pub bump: u8,                   //1
}

impl Package {

    pub fn init(&mut self, id: u16, price: u16, bump: u8) -> Result<()> {
        self.id = id;
        self.price = price;
        self.bump = bump;
        self.status = true;
        Ok(())
    }
    

   
}
