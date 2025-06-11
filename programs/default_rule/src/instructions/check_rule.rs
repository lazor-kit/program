use anchor_lang::prelude::*;

use crate::{error::RuleError, state::Rule};

pub fn check_rule(_ctx: Context<CheckRule>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CheckRule<'info> {
    pub smart_wallet_authenticator: Signer<'info>,

    #[account(
        mut,
        constraint = smart_wallet_authenticator.key() == rule.admin @ RuleError::UnAuthorize,
    )]
    pub rule: Account<'info, Rule>,
}
