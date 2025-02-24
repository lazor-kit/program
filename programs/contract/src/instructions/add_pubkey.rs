use anchor_lang::prelude::*;

use crate::{error::ContractError, ID};

use super::SmartWallet;

pub fn add_pubkey(ctx: Context<AddPubkey>, vec_pubkey: Vec<[u8; 33]>, _id: u64) -> Result<()> {
    let smart_wallet = &mut ctx.accounts.smart_wallet;

    // check if smart_wallet.pubkey.len() + vec_pubkey.len() <= 5
    if smart_wallet.authority.len() + vec_pubkey.len() > 5 {
        return Err(ContractError::TooManyPubkey.into());
    }

    for pubkey in vec_pubkey {
        smart_wallet.authority.push(pubkey);
    }

    Ok(())
}

#[derive(Accounts)]
#[instruction(_id: u64)]
pub struct AddPubkey<'info> {
    #[account(
        mut,
        seeds=[b"smart_wallet", _id.to_le_bytes().as_ref()],
        bump,
        owner = ID
    )]
    pub smart_wallet: Account<'info, SmartWallet>,
}
