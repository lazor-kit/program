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

    pub fn add_authenticator(
        ctx: Context<AddAuthenticator>,
        verify_param: VerifyParam,
        new_passkey_pubkey: PasskeyPubkey,
    ) -> Result<()> {
        instructions::add_authenticator(ctx, verify_param, new_passkey_pubkey)
    }

    // verify secp256r1 signature and execute instruction
    pub fn execute_instruction(ctx: Context<Verify>, verify_param: VerifyParam) -> Result<()> {
        instructions::execute_instruction(ctx, verify_param)
    }
}
