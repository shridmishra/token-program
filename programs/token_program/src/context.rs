use anchor_lang::prelude::*;
use anchor_spl::token::{Token,Mint,TokenAccount};



#[derive(Accounts)]
pub struct MintTokens<'info>{
    #[account(mut)]
    pub mint:Account<'info,Mint>,

    #[account(mut,
        constraint = destination.mint == mint.key(),
        constraint = destination.owner == user.key(),   
    )]

    pub destination : Account<'info,TokenAccount>,

    #[account(mut,
     seeds = [b"mint_auth",mint.key().as_ref()],
     bump
        )]
        pub mint_authority: UncheckedAccount<'info>,

        #[account(mut,signer)]
        pub user : AccountInfo<'info>,

        pub token_program: Program<'info,Token>,

}