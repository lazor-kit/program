use anchor_lang::prelude::*;

use super::PasskeyPubkey;

#[account]
#[derive(Debug, InitSpace)]
pub struct SmartWalletAuthority {
    pub pubkey: PasskeyPubkey,
    pub smart_wallet_pubkey: Pubkey,
    pub nonce: u64,
}

impl SmartWalletAuthority {
    pub const PREFIX_SEED: &'static [u8] = b"smart_wallet_authority";
}
