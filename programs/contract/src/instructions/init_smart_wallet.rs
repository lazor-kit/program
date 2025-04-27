use anchor_lang::prelude::*;

use crate::{ constants::SMART_WALLET_SEED, PasskeyExt as _, PasskeyPubkey, SmartWalletAuthority, SmartWalletData};

pub fn init_smart_wallet(ctx: Context<InitSmartWallet>, pubkey: PasskeyPubkey, id: u64) -> Result<()> {
    // Initialize the smart wallet authority
    ctx.accounts.smart_wallet_authority.set_inner( SmartWalletAuthority {
        pubkey,
        smart_wallet_pubkey: ctx.accounts.smart_wallet.key(),
        nonce: 0,
    });

    // Initialize the smart wallet data
    ctx.accounts.smart_wallet_data.set_inner( SmartWalletData {
        bump: ctx.bumps.smart_wallet,
        id,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(pubkey: PasskeyPubkey, id: u64)]
pub struct InitSmartWallet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [SMART_WALLET_SEED, id.to_le_bytes().as_ref()],
        bump,
    )]
    /// CHECK: 
    pub smart_wallet: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = SmartWalletData::DISCRIMINATOR.len() + SmartWalletData::INIT_SPACE,
        seeds = [SmartWalletData::PREFIX_SEED, smart_wallet.key().as_ref()], 
        bump,
    )]
    pub smart_wallet_data: Box<Account<'info, SmartWalletData>>,

    #[account(
        init, 
        payer = signer, 
        space = SmartWalletAuthority::DISCRIMINATOR.len() + SmartWalletAuthority::INIT_SPACE, 
        seeds = [&pubkey.to_hashed_bytes(smart_wallet.key())], 
        bump
    )]
    pub smart_wallet_authority: Box<Account<'info, SmartWalletAuthority>>,

    pub system_program: Program<'info, System>,
}
