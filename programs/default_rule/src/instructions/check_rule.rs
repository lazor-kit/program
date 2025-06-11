use anchor_lang::prelude::*;

use crate::{error::DefaultRuleError, state::Rule};

pub fn check_rule(_ctx: Context<CheckRule>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CheckRule<'info> {
    pub smart_wallet_authenticator: Signer<'info>,

    #[account(
        constraint = smart_wallet_authenticator.key() == rule.admin @ DefaultRuleError::InvalidAuthenticator,
    )]
    pub rule: Account<'info, Rule>,
}
