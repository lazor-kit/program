use anchor_lang::{prelude::*, solana_program::hash::hash};

#[derive(Debug, AnchorSerialize, AnchorDeserialize, InitSpace, PartialEq, Clone, Copy)]
pub struct PasskeyPubkey {
    pub data: [u8; 33],
}

pub trait PasskeyExt {
    fn to_hashed_bytes(&self, smart_wallet: Pubkey) -> [u8; 32];
}

impl PasskeyExt for PasskeyPubkey {
    fn to_hashed_bytes(&self, smart_wallet: Pubkey) -> [u8; 32] {
        let mut bytes = [0u8; 65];
        bytes[..33].copy_from_slice(&self.data);
        bytes[33..].copy_from_slice(&smart_wallet.to_bytes());
        let hash = hash(&bytes);
        hash.to_bytes()
    }
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
