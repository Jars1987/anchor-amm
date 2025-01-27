use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64,
   // pub authority: Pubkey,
    pub mint_x: Pubkey,
    pub mint_y: Pubkey,
    pub fee: u16, // 0% => 0, 100% => 10000
    pub locked: bool,
    pub bump: u8, //config bump
    pub mint_lp_bump: u8,
}

