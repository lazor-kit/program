use anchor_lang::prelude::*;

use crate::{
    constants::AUTHORITY_SEED,
    state::{Config, SmartWalletSeq, WhitelistRulePrograms},
};

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let whitelist_rule_programs = &mut ctx.accounts.whitelist_rule_programs;

    whitelist_rule_programs.set_inner(WhitelistRulePrograms {
        list: vec![ctx.accounts.default_rule_program.key()],
        bump: ctx.bumps.whitelist_rule_programs,
    });

    let smart_wallet_seq = &mut ctx.accounts.smart_wallet_seq;

    smart_wallet_seq.set_inner(SmartWalletSeq {
        seq: 0,
        bump: ctx.bumps.smart_wallet_seq,
    });

    let config = &mut ctx.accounts.config;

    config.set_inner(Config {
        create_smart_wallet_fee: 0,
        default_rule_program: ctx.accounts.default_rule_program.key(),
        authority_bump: ctx.bumps.authority,
        bump: ctx.bumps.config,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [Config::PREFIX_SEED],
        bump,
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = WhitelistRulePrograms::DISCRIMINATOR.len() + WhitelistRulePrograms::INIT_SPACE,
        seeds = [WhitelistRulePrograms::PREFIX_SEED],
        bump
    )]
    pub whitelist_rule_programs: Box<Account<'info, WhitelistRulePrograms>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = SmartWalletSeq::DISCRIMINATOR.len() + SmartWalletSeq::INIT_SPACE,
        seeds = [SmartWalletSeq::PREFIX_SEED],
        bump
    )]
    pub smart_wallet_seq: Box<Account<'info, SmartWalletSeq>>,

    #[account(
        init_if_needed,
        payer = payer,
        space = 0,
        seeds = [AUTHORITY_SEED],
        bump,
    )]
    /// CHECK: Only used for key and seeds.
    pub authority: UncheckedAccount<'info>,

    /// CHECK: Default Rule Program
    #[account(
        address = config.default_rule_program,
        executable
    )]
    pub default_rule_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}
