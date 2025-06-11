use crate::state::Rule;
use anchor_lang::prelude::*;

pub fn init_rule(ctx: Context<InitRule>) -> Result<()> {
    let rule = &mut ctx.accounts.rule;

    rule.set_inner(Rule {
        smart_wallet: ctx.accounts.smart_wallet.key(),
        admin: ctx.accounts.smart_wallet_authenticator.key(),
        is_initialized: false,
        bump: ctx.bumps.rule,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct InitRule<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Smart Wallet
    pub smart_wallet: UncheckedAccount<'info>,

    pub smart_wallet_authenticator: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Rule::DISCRIMINATOR.len() + Rule::INIT_SPACE,
        seeds = [Rule::PREFIX_SEED, smart_wallet.key().as_ref()],
        bump,
    )]
    pub rule: Account<'info, Rule>,

    pub system_program: Program<'info, System>,
}
