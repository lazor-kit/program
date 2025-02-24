use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;

pub mod util;
use instructions::*;
pub use util::*;

declare_id!("3jq9oBWGCUWmBynC8TTBL9KWJdGegsChJ1c8ksybGhum");

#[program]
pub mod contract {

    use super::*;

    // Initialize the smart wallet
    pub fn init_smart_wallet(
        ctx: Context<InitSmartWallet>,
        pubkey: [u8; 33],
        id: u64,
    ) -> Result<()> {
        instructions::init_smart_wallet(ctx, pubkey, id)
    }

    // verify secp256r1 signature and execute instruction
    pub fn verify_and_execute_instruction<'info>(
        ctx: Context<Verify>,
        pubkey: [u8; 33],
        msg: Vec<u8>,
        sig: [u8; 64],
        data: Vec<u8>,
    ) -> Result<()> {
        instructions::verify_and_execute_instruction(ctx, pubkey, msg, sig, data)
    }
}
