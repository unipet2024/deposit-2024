use anchor_lang::prelude::*;

use crate::AuthRole;

#[account]
pub struct AuthorityRole {
    pub bump: u8,//1
    pub status: bool,//1
    pub owner: Pubkey, //4 + 32 =
    pub role: AuthRole,
}
impl AuthorityRole {
    pub fn initialize(
        &mut self,
        authorities: &Pubkey,
        bump: u8,
        role: AuthRole,
    ) -> Result<()> {
        self.owner = *authorities;
        self.bump = bump;
        self.status = true;
        self.role = role;
        Ok(())
    }





    pub fn is_authority(&self, authority: &Pubkey) -> bool {
        self.owner == *authority
    }

    pub fn set_status_account(&mut self, status: bool) {
        self.status = status;
    }

    pub fn set_role(&mut self, role: AuthRole) {
        self.role = role;
    }
}
