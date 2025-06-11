use anchor_lang::prelude::*;

#[account(discriminator = 1)]
#[derive(Debug, InitSpace)]
pub struct Rule {
    pub smart_wallet: Pubkey,
    pub admin: Pubkey,
    pub is_initialized: bool,
}

#[account(discriminator = 2)]
#[derive(Debug, InitSpace)]
pub struct Config {
    pub authority: Pubkey,
}
