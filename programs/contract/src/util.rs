use crate::error::{ContractError};
use anchor_lang::{prelude::*, solana_program::instruction::Instruction};

const SECP256R1_ID: Pubkey = pubkey!("Secp256r1SigVerify1111111111111111111111111");

pub fn verify_secp256r1_ix(ix: &Instruction, pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
    if ix.program_id       != SECP256R1_ID                 ||  // The program id we expect
        ix.accounts.len()   != 0                            ||  // With no context accounts
        ix.data.len()       != (2 + 14 + 33 + 64 + msg.len())
    // And data of this size
    {
        return Err(ContractError::SigVerificationFailed.into()); // Otherwise, we can already throw err
    }

    check_secp256r1_data(&ix.data, pubkey, msg, sig)?; // If that's not the case, check data

    Ok(())
}

fn check_secp256r1_data(data: &[u8], pubkey: &[u8], msg: &[u8], sig: &[u8]) -> Result<()> {
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
    let msg_len: u16 = msg.len() as u16;
    let pubkey_len: u16 = pubkey.len() as u16;
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

    if &data_pubkey[..] != &pubkey[..] || &data_sig[..] != &sig[..] || &data_msg[..] != &msg[..] {
        return Err(ContractError::SigVerificationFailed.into());
    }
    Ok(())
}
