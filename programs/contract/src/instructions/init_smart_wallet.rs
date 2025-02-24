use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct SmartWallet {
    pub creator: [u8; 33],

    #[max_len(5)]
    pub authority: Vec<[u8; 33]>,

    pub id: u64,

    pub bump: u8,
}

impl SmartWallet {
    pub const PREFIX_SEED : &'static [u8] = b"smart_wallet";
}

pub fn init_smart_wallet(ctx: Context<InitSmartWallet>, pubkey: [u8;33], id: u64) -> Result<()> {
    let smart_wallet = &mut ctx.accounts.smart_wallet;
    
    // Initialize the smart wallet
    smart_wallet.creator = pubkey;
    smart_wallet.id = id;
    smart_wallet.bump = ctx.bumps.smart_wallet;

    Ok(())
}


#[derive(Accounts)]
#[instruction(pubkey: [u8;33], id: u64)]
pub struct InitSmartWallet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init, 
        payer = signer, 
        space = 8 + SmartWallet::INIT_SPACE, 
        seeds = [SmartWallet::PREFIX_SEED, &id.to_le_bytes()], 
        bump
    )]
    pub smart_wallet: Account<'info, SmartWallet>,

    pub system_program: Program<'info, System>,
}
