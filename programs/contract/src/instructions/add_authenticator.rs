use anchor_lang::{prelude::*, solana_program::sysvar::instructions::ID as IX_ID};

use crate::{constants::SMART_WALLET_SEED, verify_authority, PasskeyExt as _, PasskeyPubkey, SmartWalletAuthority, SmartWalletData, VerifyParam, ID};

pub fn add_authenticator(ctx: Context<AddAuthenticator>, verify_param: VerifyParam, new_passkey: PasskeyPubkey) -> Result<()> {
    let smart_wallet = &ctx.accounts.smart_wallet;
    let smart_wallet_authority = &mut ctx.accounts.smart_wallet_authority;
    let new_smart_wallet_authority = &mut ctx.accounts.new_wallet_authority;

    verify_authority(
        0,
        &ctx.accounts.ix_sysvar,
        &verify_param,
        smart_wallet_authority.nonce,
        smart_wallet_authority.pubkey.clone(),
    )?;

    // assert_eq!(new_passkey_pubkey, new_passkey.data);
    
    new_smart_wallet_authority.nonce = 0;
    new_smart_wallet_authority.pubkey = PasskeyPubkey {
        data: new_passkey.data,
    };
    new_smart_wallet_authority.smart_wallet_pubkey = smart_wallet.key();
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(verify_param: VerifyParam, new_passkey: PasskeyPubkey)]
pub struct AddAuthenticator<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = IX_ID)]
    /// CHECK: 
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

    #[account(
        init,
        payer = payer,
        space = SmartWalletAuthority::DISCRIMINATOR.len() + SmartWalletAuthority::INIT_SPACE,
        seeds = [&new_passkey.to_hashed_bytes(smart_wallet.key())], 
        bump
    )]
    pub new_wallet_authority: Account<'info, SmartWalletAuthority>,

    pub system_program: Program<'info, System>,
}
