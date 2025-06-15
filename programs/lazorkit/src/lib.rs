use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use constants::PASSKEY_SIZE;
use instructions::*;

declare_id!("HPN843A4SZB7tfcLF9pu6hbvwTgv7HtdRscXoZWbAdXs");

/// The Lazor Kit program provides smart wallet functionality with passkey authentication
#[program]
pub mod lazorkit {
    use super::*;

    /// Initialize the program by creating the sequence tracker
    #[instruction(discriminator = 1)]
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// Create a new smart wallet with passkey authentication
    #[instruction(discriminator = 2)]
    pub fn create_smart_wallet(
        ctx: Context<CreateSmartWallet>,
        passkey_pubkey: [u8; PASSKEY_SIZE],
        rule_data: Vec<u8>,
    ) -> Result<()> {
        instructions::create_smart_wallet(ctx, passkey_pubkey, rule_data)
    }

    /// Execute an instruction with passkey authentication
    #[instruction(discriminator = 3)]
    pub fn execute_instruction(
        ctx: Context<ExecuteInstruction>,
        args: ExecuteInstructionArgs,
    ) -> Result<()> {
        instructions::execute_instruction(ctx, args)
    }

    /// Update the list of whitelisted rule programs
    #[instruction(discriminator = 4)]
    pub fn upsert_whitelist_rule_programs(
        ctx: Context<UpsertWhitelistRulePrograms>,
        program_id: Pubkey,
    ) -> Result<()> {
        instructions::upsert_whitelist_rule_programs(ctx, program_id)
    }
}
