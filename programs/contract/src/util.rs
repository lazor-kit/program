use crate::{error::ContractError, PasskeyPubkey, VerifyParam};
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, sysvar::instructions::load_instruction_at_checked},
};

const SECP256R1_ID: Pubkey = pubkey!("Secp256r1SigVerify1111111111111111111111111");

pub fn verify_secp256r1_ix(ix: &Instruction, verify_params: &VerifyParam) -> Result<()> {
    if ix.program_id       != SECP256R1_ID                 ||  // The program id we expect
        ix.accounts.len()   != 0                            ||  // With no context accounts
        ix.data.len()       != (2 + 14 + 33 + 64 + verify_params.msg.try_to_vec()?.len())
    // And data of this size
    {
        return Err(ContractError::SigVerificationFailed.into()); // Otherwise, we can already throw err
    }

    check_secp256r1_data(&ix.data, verify_params)?; // If that's not the case, check data

    Ok(())
}

fn check_secp256r1_data(data: &[u8], verify_params: &VerifyParam) -> Result<()> {
    let VerifyParam { pubkey, msg, sig } = verify_params;

    // Parse header components
    let num_signatures = &[data[0]]; // Byte 0
    let signature_offset = &data[2..=3]; // Bytes 2-3
    let signature_instruction_index = &data[4..=5]; // Bytes 4-5
    let public_key_offset = &data[6..=7]; // Bytes 6-7
    let public_key_instruction_index = &data[8..=9]; // Bytes 8-9
    let message_data_offset = &data[10..=11]; // Bytes 10-11
    let message_data_size = &data[12..=13]; // Bytes 12-13
    let message_instruction_index = &data[14..=15]; // Bytes 14-15

    // Get actual data
    let data_pubkey = &data[16..16 + 33]; // 33 bytes public key
    let data_sig = &data[49..49 + 64]; // 64 bytes signature
    let data_msg = &data[113..]; // Variable length message

    // Calculate expected values
    const SIGNATURE_OFFSETS_SERIALIZED_SIZE: u16 = 14;
    const DATA_START: u16 = 2 + SIGNATURE_OFFSETS_SERIALIZED_SIZE;
    let msg_len: u16 = msg.try_to_vec()?.len() as u16;
    let pubkey_len: u16 = pubkey.data.len() as u16;
    let sig_len: u16 = sig.len() as u16;

    let exp_pubkey_offset: u16 = DATA_START;
    let exp_signature_offset: u16 = DATA_START + pubkey_len;
    let exp_message_data_offset: u16 = exp_signature_offset + sig_len;

    // Verify header
    if num_signatures != &[1]
        || signature_offset != &exp_signature_offset.to_le_bytes()
        || signature_instruction_index != &0xFFFFu16.to_le_bytes()
        || public_key_offset != &exp_pubkey_offset.to_le_bytes()
        || public_key_instruction_index != &0xFFFFu16.to_le_bytes()
        || message_data_offset != &exp_message_data_offset.to_le_bytes()
        || message_data_size != &msg_len.to_le_bytes()
        || message_instruction_index != &0xFFFFu16.to_le_bytes()
    {
        return Err(ContractError::SigVerificationFailed.into());
    }

    if &data_pubkey[..] != &pubkey.data[..]
        || &data_sig[..] != &sig[..]
        || &data_msg[..] != &msg.try_to_vec()?[..]
    {
        return Err(ContractError::SigVerificationFailed.into());
    }
    Ok(())
}

pub fn verify_authority<'info>(
    instruction_index: u8,
    instruction_sysvar_account_info: &AccountInfo<'info>,
    verify_params: &VerifyParam,
    expected_nonce: u64,
    expected_pubkey: PasskeyPubkey,
) -> Result<Vec<u8>> {
    let ix: Instruction =
        load_instruction_at_checked(instruction_index as usize, instruction_sysvar_account_info)?;

    verify_secp256r1_ix(&ix, verify_params)?;

    let VerifyParam {
        msg,
        pubkey: _,
        sig: _,
    } = verify_params;

    let clock = Clock::get()?;

    let current_time = clock.unix_timestamp;

    // check if timestamp is in the future
    if msg.timestamp > current_time + 30 * 1000 {
        return Err(ContractError::InvalidTimestamp.into());
    }

    // check if timestamp is expired in 30 seconds
    if current_time > msg.timestamp + 30 * 1000 {
        return Err(ContractError::SignatureExpired.into());
    }

    // check if nonce is the same
    if msg.nonce != expected_nonce {
        return Err(ContractError::InvalidNonce.into());
    }

    // Check that pubkey is the creator of the smart wallet
    if verify_params.pubkey != expected_pubkey {
        return Err(ContractError::InvalidPubkey.into());
    }

    Ok(msg.payload.clone())
}
