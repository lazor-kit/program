use anchor_lang::prelude::*;

#[derive(Default, InitSpace, Clone, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub enum MemberType {
    Admin,
    #[default]
    Member,
}

#[account(discriminator = 2)]
#[derive(Default, InitSpace)]
pub struct Member {
    pub owner: Pubkey,
    pub member_type: MemberType,
    pub smart_wallet: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
}

impl Member {
    pub const PREFIX_SEED: &'static [u8] = b"member";
}
