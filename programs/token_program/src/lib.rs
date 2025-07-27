use anchor_lang::prelude::*;
use anchor_spl::token;

pub mod constants;
pub mod state;
pub mod processor;
pub mod context;



declare_id!("Cb48TcW2JXHh9Vf5sGYTuUKGUnxcugSof2AKc2he69uh");

#[program]
pub mod token_program {
    use super::*;
    use context:: MintTokens;

    pub fn mint_token(ctx: Context<MintTokens>, amount: u64) -> Result<>{
        processor::mint_token(ctx,amount);


    }

   
}

