use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use super::shared::{close_token_account, transfer_tokens};
use crate::{error::ErrorCode, state::Offer};

#[derive(Accounts)]
pub struct RefundOffer<'info> {
    // Work with either the classic token program or
    // the newer token extensions program
    pub token_program: Interface<'info, TokenInterface>,

    // Used to create accounts
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub maker: Signer<'info>,

    pub token_mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = maker,
        has_one = maker,
        seeds = [b"offer", offer.id.to_le_bytes().as_ref()],
        bump = offer.bump
    )]
    pub offer: Account<'info, Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
}

// Handle the refund offer instruction by:
// 1. Returning the tokens from the vault to the maker's account
// 2. Closing the vault and returning the rent to the maker
pub fn refund_offer(ctx: Context<RefundOffer>) -> Result<()> {
    let offer_account_seeds = &[
        b"offer",
        &ctx.accounts.offer.id.to_le_bytes()[..],
        &[ctx.accounts.offer.bump],
    ];
    let signers_seeds = Some(&offer_account_seeds[..]);

    // Return the tokens from the vault to the maker's account
    transfer_tokens(
        &ctx.accounts.vault,
        &ctx.accounts.maker_token_account_a,
        &ctx.accounts.vault.amount,
        &ctx.accounts.token_mint_a,
        &ctx.accounts.offer.to_account_info(),
        &ctx.accounts.token_program,
        signers_seeds,
    )
    .map_err(|_| ErrorCode::FailedRefundTransfer)?;

    // Close the vault and return the rent to the maker
    close_token_account(
        &ctx.accounts.vault,
        &ctx.accounts.maker.to_account_info(),
        &ctx.accounts.offer.to_account_info(),
        &ctx.accounts.token_program,
        signers_seeds,
    )
    .map_err(|_| ErrorCode::FailedRefundClosure)?;

    Ok(())
}