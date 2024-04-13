use anchor_lang::prelude::*;

// total 230
#[account]
pub struct Package {
    pub packages: Vec<u64>, // 6
    pub valid: bool,        //1
    pub bump: u8,           //1
}

impl Package {
    pub fn set_packages(&mut self, packages: &Vec<u64>) -> Result<()> {
        self.packages = vec![];

        for package in packages.iter() {
            self.add_package(*package)?;
        }
        Ok(())
    }

    fn add_package(&mut self, package: u64) -> Result<()> {
        self.packages.push(package);
        Ok(())
    }

    pub fn get_package(&self, id: u64) -> u64 {
        if id > self.packages.len() as u64 - 1 {
            return 0;
        }
        self.packages[id as usize]
    }
}
