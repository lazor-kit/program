use anchor_lang::prelude::*;

#[account(discriminator = 1)]
#[derive(Debug, InitSpace)]
pub struct Config {
    pub authority: Pubkey,
}

impl Config {
    pub const PREFIX_SEED: &'static [u8] = b"config";
}
