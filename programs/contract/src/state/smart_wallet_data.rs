use anchor_lang::prelude::*;

#[derive(Debug, AnchorSerialize, AnchorDeserialize, InitSpace, PartialEq, Clone)]
pub struct PasskeyPubkey {
    pub data: [u8; 33],
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Message {
    pub nonce: u64,
    pub timestamp: i64,
    pub payload: Vec<u8>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct VerifyParam {
    pub pubkey: PasskeyPubkey,
    pub msg: Message,
    pub sig: [u8; 64],
}

#[account]
#[derive(Debug, InitSpace)]
pub struct SmartWalletData {
    // The bump seed for the smart wallet account
    pub bump: u8,

    // The ID of the smart wallet
    pub id: u64,
}

impl SmartWalletData {
    pub const PREFIX_SEED: &'static [u8] = b"smart_wallet_data";
}

pub const SMART_WALLET_SEED: &'static [u8] = b"smart_wallet";
