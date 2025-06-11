use crate::error::RuleError;
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
        constraint = smart_wallet_authenticator.key() == rule.admin @ RuleError::UnAuthorize,
        constraint = rule.smart_wallet == smart_wallet.key() @ RuleError::UnAuthorize,
        close = smart_wallet
    )]
    pub rule: Account<'info, Rule>,
}
