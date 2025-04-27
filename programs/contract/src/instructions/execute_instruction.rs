use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use crate::constants::SMART_WALLET_SEED;
use crate::{ verify_authority, PasskeyExt as _, SmartWalletAuthority, SmartWalletData, VerifyParam, ID};
use anchor_lang::solana_program::sysvar::instructions::ID as IX_ID;
use anchor_lang::solana_program::instruction::Instruction;


pub fn execute_instruction(
    ctx: Context<Verify>,
    verify_param: VerifyParam,
    instruction_data: Vec<u8>,
) -> Result<()> {
    let smart_wallet = &ctx.accounts.smart_wallet;
    let smart_wallet_data = &ctx.accounts.smart_wallet_data;
    let smart_wallet_authority = &mut ctx.accounts.smart_wallet_authority;
    let cpi_program_key = &ctx.accounts.cpi_program;

    verify_authority(
        0,
        &ctx.accounts.ix_sysvar,
        &verify_param,
        smart_wallet_authority.nonce,
        smart_wallet_authority.pubkey.clone(),
    )?;

    let seeds: &[&[u8]] = &[SMART_WALLET_SEED, &smart_wallet_data.id.to_le_bytes()];

    let seeds_signer = &mut seeds.to_vec();
    let binding = [smart_wallet_data.bump];
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
        data: instruction_data,
    };

    // Execute the instruction
    invoke_signed(&instruction, &ctx.remaining_accounts, &[seeds_signer])?;

    // Increment nonce
    smart_wallet_authority.nonce += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(verify_param: VerifyParam)]
pub struct Verify<'info> {
    /// CHECK: The address check is needed because otherwise
    /// the supplied Sysvar could be anything else.
    /// The Instruction Sysvar has not been implemented
    /// in the Anchor framework yet, so this is the safe approach.
    #[account(address = IX_ID)]
    pub ix_sysvar: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SMART_WALLET_SEED, &smart_wallet_data.id.to_le_bytes()],
        bump = smart_wallet_data.bump,
        owner = ID,
    )]
    /// CHECK:
    pub smart_wallet: UncheckedAccount<'info>,

    #[account(
        seeds = [SmartWalletData::PREFIX_SEED, smart_wallet.key().as_ref()], 
        bump,
        owner = ID
    )]
    pub smart_wallet_data: Account<'info, SmartWalletData>,

    
    #[account(
        mut,
        seeds = [&verify_param.pubkey.to_hashed_bytes(smart_wallet.key())], 
        bump,
        owner = ID,
    )]
    pub smart_wallet_authority: Account<'info, SmartWalletAuthority>,

    /// CHECK:
    pub cpi_program: AccountInfo<'info>,
}
