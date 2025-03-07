use anchor_lang::prelude::*;

use crate::{smart_wallet_authority, PasskeyPubkey, SmartWalletAuthority, ID};

#[derive(Debug, AnchorSerialize, AnchorDeserialize)]
pub struct AddPubkeyMessage {
    pub nonce: u64,
    pub timestamp: i64,
    pub id: u64,
}

// pub fn add_pubkey(
//     ctx: Context<AddPubkey>,
//     pubkey: PasskeyPubkey,
//     msg: AddPubkeyMessage,
//     sig: [u8; 64],
// ) -> Result<()> {
//     let smart_wallet_authority = &mut ctx.accounts.smart_wallet_authority;
//     let new_authority = &ctx.accounts.new_authority;

//     Ok(())
// }

#[derive(Accounts)]
#[instruction(pubkey: PasskeyPubkey, msg: AddPubkeyMessage)]
pub struct AddPubkey<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        owner = ID,
    )]
    pub smart_wallet_authority: Account<'info, SmartWalletAuthority>,

    #[account(
        init,
        seeds = [SmartWalletAuthority::PREFIX_SEED, smart_wallet_authority.smart_wallet_pubkey.as_ref(), msg.id.to_le_bytes().as_ref()],
        bump,
        space = 8 + SmartWalletAuthority::INIT_SPACE,
        payer = payer,
    )]
    pub new_authority: Account<'info, SmartWalletAuthority>,

    pub system_program: Program<'info, System>,
}
