use anchor_lang::prelude::*;
use anchor_spl::token::{self, spl_token::error, Burn, Mint, MintTo, Token, TokenAccount};

declare_id!("7VB9bByo3zkshwzUEN4DBq9EQKMedSU81gdFyGVQgAHg");

#[program]
pub mod furb {
    use solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> ProgramResult {
        let mint = &ctx.accounts.mint;
        let furb = &ctx.accounts.furb;
        let furb_mint = &ctx.accounts.furb_mint;

        token::mint_to(ctx.accounts.into(), amount)?;

        furb.mint = *mint.to_account_info().key;
        furb.mint_authority = *furb_mint.to_account_info().key;

        Ok(())
    }

    pub fn burn(ctx: Context<Burn>, amount: u64) -> ProgramResult {
        Ok(token::burn(ctx.accounts.into(), amount)?)
    }
}
