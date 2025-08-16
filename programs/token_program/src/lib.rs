use anchor_lang::prelude::*;


pub mod constants;
pub mod context;
pub mod processor;
pub mod state;

use context::{BurnTokens, MintTokens, TransferTokens};

declare_id!("Cb48TcW2JXHh9Vf5sGYTuUKGUnxcugSof2AKc2he69uh");

// #[program]
pub mod token_program {
    use super::*;

    /// Mint `amount` tokens to `destination`, with the PDA as mint authority.
    pub fn mint_token(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        processor::mint_token(ctx, amount)
    }

    /// Burn `amount` tokens from the user's `from` token account.
    pub fn burn_token(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        processor::burn_token(ctx, amount)
    }

    /// Transfer `amount` tokens from user's `from` to `to`.
    pub fn transfer_token(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        processor::transfer_token(ctx, amount)
    }
}
