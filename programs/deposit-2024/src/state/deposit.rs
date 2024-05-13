

use anchor_lang::prelude::*;

use crate::DepositErrors;

// use crate::DepositStatus;

// total 230
#[account]
pub struct Deposit {
    pub operator_wallet: Pubkey, //wallet with draw balance deposit
    pub support_currency: Vec<Pubkey>, //list token mint
    pub admin_role: Vec<Pubkey>,
    pub bump: u8,   
    pub version: String ,    
    pub packages: Vec<u16> ,    //1
}


impl Deposit {
    pub fn init(
        &mut self,
        admin: Pubkey,
        operator: Pubkey,
        bump: u8, // tokens: &Vec<Pubkey>,
    ) ->  Result<()>  {

        self.operator_wallet = operator;
        self.bump = bump;
        self.admin_role = vec![admin];
        self.version = "1.0.0".to_string();
        self.support_currency = vec![Pubkey::default()];
        Ok(())
    }
    pub fn set_admin(&mut self, admin: Pubkey) -> Result<()> {
        
        if self.admin_role.contains(&admin) || self.admin_role.len() > 3{
            return Err(DepositErrors::AdminAccountExisted.into());
        }
        self.admin_role.push(admin);
        Ok(())
    }
    pub fn is_admin(&self, admin: Pubkey) -> bool {
        self.admin_role.contains(&admin)
    }

    pub fn get_admin(&self) -> Vec<Pubkey> {
        self.admin_role.clone()
    }
    
    pub fn get_operator(&self) -> Pubkey {
        self.operator_wallet
    }

    pub fn remove_admin(&mut self, admin: Pubkey) ->Result<()> {
        if self.admin_role.contains(&admin) {
            self.admin_role.retain(|&x| x != admin);
        }
        Ok(())
    }
    pub fn set_operator(&mut self, operator: Pubkey) ->Result<()>{
        self.operator_wallet = operator;
        Ok(())
    }

    pub fn add_currency(&mut self, currency: Pubkey) -> Result<()> {
        if self.support_currency.contains(&currency) {
            return Err(DepositErrors::CurrencyExisted.into());
        }
        self.support_currency.push(currency);
        Ok(())
    }

    pub fn remove_currency(&mut self, currency: Pubkey) -> Result<()> {
        if self.support_currency.contains(&currency) {
            self.support_currency.retain(|&x| x != currency);
        }
        Ok(())
    }

    pub fn add_package(&mut self, package: u16) -> Result<()> {
        if self.packages.contains(&package) {
            return Err(DepositErrors::PackageExisted.into());
        }
        self.packages.push(package);
        Ok(())
    }

    pub fn is_support_currency(&self, currency: Pubkey) -> bool {
        self.support_currency.contains(&currency)
    }

    
}
