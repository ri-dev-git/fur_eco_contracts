use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};
declare_id!("7VB9bByo3zkshwzUEN4DBq9EQKMedSU81gdFyGVQgAHg");

#[program]
pub mod furb {

    use super::*;

    pub fn initialize(ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Created Mint Account: {:?}", ctx.accounts.mint.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
