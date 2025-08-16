use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, Transfer};
use crate::constants::MINT_AUTHORITY_SEED;
use crate::context::{BurnTokens, MintTokens, TransferTokens};

/// Mint via CPI to SPL Token, using PDA as authority.
pub fn mint_token(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // 1) Read bump as a local u8 (Anchor generates ctx.bumps.<name>)
    let bump: u8 = ctx.bumps.mint_authority;

    // 2) Bind the mint pubkey to a local variable so its .as_ref() slice lives long enough
    let mint_key = ctx.accounts.mint.key();

    // 3) Prepare a small array for bump so we can take a slice reference to it
    let bump_seed: [u8; 1] = [bump];

    // 4) Now construct signer_seeds using the bound values (all live for the call)
    let signer_seeds: [&[u8]; 3] = [
        MINT_AUTHORITY_SEED,
        mint_key.as_ref(),
        &bump_seed[..],
    ];

    // 5) Build CPI context with PDA signer
    let binding = [&signer_seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.destination.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        },
        // pass reference to the array of slices
        &binding,
    );

    // 6) Execute CPI
    token::mint_to(cpi_ctx, amount)?;
    Ok(())
}

/// Burn tokens from the caller's token account.
pub fn burn_token(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    // Authority is the user (signer) — no PDA involved here
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    token::burn(cpi_ctx, amount)?;
    Ok(())
}

/// Transfer tokens between token accounts owned by the user.
pub fn transfer_token(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
    // Authority is the user (signer)
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
    );

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}
