use anchor_lang::prelude::*;

use crate::{ PasskeyPubkey, SmartWalletAuthority, SmartWalletData, SMART_WALLET_SEED};

pub fn init_smart_wallet(ctx: Context<InitSmartWallet>, pubkey: PasskeyPubkey, id: u64) -> Result<()> {
    let smart_wallet_authority = &mut ctx.accounts.smart_wallet_authority;
    let smart_wallet = &ctx.accounts.smart_wallet;
    let smart_wallet_data = &mut ctx.accounts.smart_wallet_data;
    
    // Initialize the smart wallet authority
    smart_wallet_authority.pubkey = pubkey;
    smart_wallet_authority.smart_wallet_pubkey = smart_wallet.key();
    smart_wallet_authority.nonce = 0;

    // Initialize the smart wallet data
    smart_wallet_data.id = id;
    smart_wallet_data.bump = ctx.bumps.smart_wallet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(pubkey: PasskeyPubkey, id: u64)]
pub struct InitSmartWallet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 0,
        seeds = [SMART_WALLET_SEED, id.to_le_bytes().as_ref()],
        bump,
    )]
    /// CHECK: 
    pub smart_wallet: UncheckedAccount<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + SmartWalletData::INIT_SPACE,
        seeds = [SmartWalletData::PREFIX_SEED, smart_wallet.key().as_ref()], 
        bump,
    )]
    pub smart_wallet_data: Account<'info, SmartWalletData>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + SmartWalletAuthority::INIT_SPACE, 
        seeds = [SmartWalletAuthority::PREFIX_SEED, smart_wallet.key().as_ref(), id.to_le_bytes().as_ref()], 
        bump
    )]
    pub smart_wallet_authority: Account<'info, SmartWalletAuthority>,

    pub system_program: Program<'info, System>,
}
