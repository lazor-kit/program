use anchor_lang::prelude::*;

#[account(discriminator = 1)]
#[derive(Debug, InitSpace)]
pub struct Rule {
    pub smart_wallet: Pubkey,
    pub admin: Pubkey,
    pub is_initialized: bool,
}

impl Rule {
    pub const PREFIX_SEED: &'static [u8] = b"rule";
}

#[account(discriminator = 2)]
#[derive(Debug, InitSpace)]
pub struct Config {
    pub authority: Pubkey,
}

impl Config {
    pub const PREFIX_SEED: &'static [u8] = b"config";
}
