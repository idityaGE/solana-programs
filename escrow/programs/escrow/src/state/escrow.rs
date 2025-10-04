use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    // Indentifier of the offer
    pub id: u64,
    // who made the offer
    pub maker: Pubkey,
    // The token mint of the token being offered
    pub token_mint_a: Pubkey,
    // The token mint of the token wanted
    pub token_mint_b: Pubkey,
    // The amount of token b being wanted
    pub token_b_wanted_amount: u64,
    pub bump: u8,
}
