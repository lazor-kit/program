use crate::errors::DefaultRuleError;
use crate::state::Rule;
use anchor_lang::prelude::*;

pub fn destroy(_ctx: Context<Destroy>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Destroy<'info> {
    /// CHECK: Smart Wallet
    pub smart_wallet: UncheckedAccount<'info>,
    pub smart_wallet_authenticator: Signer<'info>,
    #[account(
        mut,
        seeds = [Rule::PREFIX_SEED, smart_wallet.key().as_ref()],
        bump = rule.bump,
        constraint = smart_wallet_authenticator.key() == rule.admin @ DefaultRuleError::InvalidAuthenticator,
        has_one = smart_wallet @ DefaultRuleError::InvalidSmartWallet,
        close = smart_wallet
    )]
    pub rule: Account<'info, Rule>,
}
