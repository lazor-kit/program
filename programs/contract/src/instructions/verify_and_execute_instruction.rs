use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;

use crate::error::ContractError;
use crate::util::verify_secp256r1_ix;
use crate::ID;
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;
use anchor_lang::solana_program::{
    instruction::Instruction, sysvar::instructions::load_instruction_at_checked,
};

use super::SmartWallet;

pub fn verify_and_execute_instruction<'info>(
    ctx: Context<Verify>,
    pubkey: [u8; 33],
    msg: Vec<u8>,
    sig: [u8; 64],
    data: Vec<u8>,
) -> Result<()> {
    let smart_wallet = &ctx.accounts.smart_wallet;
    let cpi_program_key = &ctx.accounts.cpi_program;

    // Get what should be the Secp256k1Program instruction
    let ix: Instruction = load_instruction_at_checked(1, &ctx.accounts.ix_sysvar)?;

    // Check that ix is what we expect to have been sent
    verify_secp256r1_ix(&ix, &pubkey, &msg, &sig)?;

    // Check that pubkey is the creator of the smart wallet
    if pubkey != smart_wallet.creator {
        return Err(ContractError::InvalidPubkey.into());
    }

    let seeds: &[&[u8]] = &[SmartWallet::PREFIX_SEED, &smart_wallet.id.to_le_bytes()];

    let seeds_signer = &mut seeds.to_vec();
    let binding = [smart_wallet.bump];
    seeds_signer.push(&binding);

    let accounts: Vec<AccountMeta> = ctx
        .remaining_accounts
        .iter()
        .map(|acc| AccountMeta {
            pubkey: *acc.key,
            is_signer: *acc.key == smart_wallet.key(), // check if pubkey equal smart_wallet_pda
            is_writable: acc.is_writable,
        })
        .collect();

    // Create instruction
    let instruction = Instruction {
        program_id: cpi_program_key.key(),
        accounts,
        data,
    };

    // Execute the instruction
    invoke_signed(&instruction, &ctx.remaining_accounts, &[seeds_signer])?;

    Ok(())
}

#[derive(Accounts)]
pub struct Verify<'info> {
    /// CHECK: The address check is needed because otherwise
    /// the supplied Sysvar could be anything else.
    /// The Instruction Sysvar has not been implemented
    /// in the Anchor framework yet, so this is the safe approach.
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    #[account(
        mut,
        owner = ID
    )]
    pub smart_wallet: Account<'info, SmartWallet>,

    /// CHECK:
    pub cpi_program: AccountInfo<'info>,
}
