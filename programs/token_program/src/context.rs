use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::constants::MINT_AUTHORITY_SEED;

/// Accounts for minting tokens using a PDA as the mint authority.
#[derive(Accounts)]
pub struct MintTokens<'info> {
    /// SPL Mint whose authority is the PDA derived below.
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// Destination token account (ATA) that receives newly minted tokens.
    #[account(
        mut,
        constraint = destination.mint == mint.key(),
        constraint = destination.owner == user.key(),
    )]
    pub destination: Account<'info, TokenAccount>,

    /// CHECK: PDA that is set as the mint authority of `mint`.
    #[account(
        seeds = [MINT_AUTHORITY_SEED, mint.key().as_ref()],
        bump
    )]
    pub mint_authority: UncheckedAccount<'info>,

    /// The user triggering the mint (payer / UI wallet).
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    /// SPL Token program.
    pub token_program: Program<'info, Token>,
}

/// Accounts for burning tokens from the caller's token account.
#[derive(Accounts)]
pub struct BurnTokens<'info> {
    /// SPL Mint for constraint checking against `from`.
    pub mint: Account<'info, Mint>,

    /// Source token account to burn from (must belong to user and match mint).
    #[account(
        mut,
        constraint = from.mint == mint.key(),
        constraint = from.owner == user.key(),
    )]
    pub from: Account<'info, TokenAccount>,

    /// The account owner authorizing the burn.
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    /// SPL Token program.
    pub token_program: Program<'info, Token>,
}

/// Accounts for transferring tokens from user’s account to someone else.
#[derive(Accounts)]
pub struct TransferTokens<'info> {
    /// SPL Mint for constraint checking.
    pub mint: Account<'info, Mint>,

    /// Sender token account (owned by `user`).
    #[account(
        mut,
        constraint = from.mint == mint.key(),
        constraint = from.owner == user.key(),
    )]
    pub from: Account<'info, TokenAccount>,

    /// Recipient token account (same mint).
    #[account(
        mut,
        constraint = to.mint == mint.key(),
    )]
    pub to: Account<'info, TokenAccount>,

    /// The owner of `from` account authorizing the transfer.
    #[account(mut, signer)]
    pub user: AccountInfo<'info>,

    /// SPL Token program.
    pub token_program: Program<'info, Token>,
}
