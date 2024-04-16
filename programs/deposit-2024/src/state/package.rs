use anchor_lang::prelude::*;

use crate::PackageItem;

// total 230
#[account]
pub struct Package {
    pub packages: Vec<PackageItem>, // 4 + n * 24
    pub valid: bool,                //1
    pub bump: u8,                   //1
}

impl Package {
    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump
    }

    pub fn set_packages(&mut self, packages: &Vec<PackageItem>) -> Result<()> {
        self.packages = vec![];

        for package in packages.iter() {
            self.add_package(&package)?;
        }
        Ok(())
    }

    fn add_package(&mut self, package: &PackageItem) -> Result<()> {
        self.packages.push(*package);
        Ok(())
    }

    pub fn get_package(&self, id: u64) -> PackageItem {
        for package_item in self.packages.iter() {
            if package_item.id == id {
                msg!("Get found: {:?}", *package_item);
                return *package_item;
            }
        }

        PackageItem {
            id: 0,
            price: 0,
            reward: 0,
        }
    }
}
