use anchor_lang::prelude::*;

declare_id!("AULUCD8kw4Nnjb1hUsWyvucZ5tzwu3wCH7Dstc5p6AMj");

mod errors;
mod instructions;
mod state;

use instructions::*;

#[program]
pub mod default_rule {

    use super::*;

    #[instruction(discriminator = 1)]
    pub fn initialize(ctx: Context<Initialize>, lazorkit_author: Pubkey) -> Result<()> {
        instructions::initialize(ctx, lazorkit_author)
    }

    #[instruction(discriminator = 2)]
    pub fn init_rule(ctx: Context<InitRule>) -> Result<()> {
        instructions::init_rule(ctx)
    }

    #[instruction(discriminator = 3)]
    pub fn check_rule(_ctx: Context<CheckRule>) -> Result<()> {
        instructions::check_rule(_ctx)
    }

    #[instruction(discriminator = 4)]
    pub fn destroy(ctx: Context<Destroy>) -> Result<()> {
        instructions::destroy(ctx)
    }
}
