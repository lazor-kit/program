use anchor_lang::prelude::*;
use lazorkit::{program::Lazorkit, state::SmartWalletAuthenticator, utils::PasskeyExt};

use crate::state::{Member, MemberType};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AddMemberArgs {
    pub member: Pubkey,
}

pub fn add_member(ctx: Context<AddMember>, _new_passkey_pubkey: [u8; 33]) -> Result<()> {
    let member = &mut ctx.accounts.member;
    let new_smart_wallet_authenticator = &mut ctx.accounts.new_smart_wallet_authenticator;
    let smart_wallet_authenticator = &mut ctx.accounts.smart_wallet_authenticator;

    member.set_inner(Member {
        owner: new_smart_wallet_authenticator.key(),
        member_type: MemberType::Member,
        smart_wallet: smart_wallet_authenticator.smart_wallet,
        bump: new_smart_wallet_authenticator.bump,
        is_initialized: true,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(new_passkey_pubkey: [u8; 33])]
pub struct AddMember<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(signer)]
    pub smart_wallet_authenticator: Account<'info, SmartWalletAuthenticator>,

    #[account(
        seeds = [&new_passkey_pubkey.to_hashed_bytes(smart_wallet_authenticator.smart_wallet.key())],
        seeds::program = lazorkit.key(),
        bump = new_smart_wallet_authenticator.bump,
    )]
    pub new_smart_wallet_authenticator: Account<'info, SmartWalletAuthenticator>,

    #[account(
        seeds = [Member::PREFIX_SEED, smart_wallet_authenticator.smart_wallet.key().as_ref(), smart_wallet_authenticator.key().as_ref()],
        bump = admin.bump,
        constraint = admin.member_type == MemberType::Admin,
    )]
    pub admin: Account<'info, Member>,

    #[account(
        init,
        payer = payer,
        space = Member::DISCRIMINATOR.len() + Member::INIT_SPACE,
        seeds = [Member::PREFIX_SEED, smart_wallet_authenticator.smart_wallet.key().as_ref(), new_smart_wallet_authenticator.key().as_ref()],
        bump,
    )]
    pub member: Account<'info, Member>,

    pub lazorkit: Program<'info, Lazorkit>,

    pub system_program: Program<'info, System>,
}
