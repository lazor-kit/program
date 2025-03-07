use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;
pub mod util;

use instructions::*;
pub use state::*;
pub use util::*;

declare_id!("3jq9oBWGCUWmBynC8TTBL9KWJdGegsChJ1c8ksybGhum");

#[program]
pub mod contract {

    use super::*;

    // Initialize the smart wallet
    pub fn init_smart_wallet(
        ctx: Context<InitSmartWallet>,
        pubkey: PasskeyPubkey,
        id: u64,
    ) -> Result<()> {
        instructions::init_smart_wallet(ctx, pubkey, id)
    }

    // verify secp256r1 signature and execute instruction
    pub fn execute_instruction(ctx: Context<Verify>, verify_params: VerifyParam) -> Result<()> {
        instructions::execute_instruction(ctx, verify_params)
    }
}
