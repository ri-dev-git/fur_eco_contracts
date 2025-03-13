use crate::state::tokens::TokenConversionVault;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub fn convert_tokens(ctx: Context<TokenConversion>, amount: u64) -> Result<()> {
    let conversion_vault = &mut ctx.accounts.conversion_vault;

    // Conversion rates: 5000 FURB = 5 FURA
    require!(amount % 1000 == 0, ErrorCode::InvalidConversionAmount);

    let fura_amount = (amount / 1000) * 1;

    // Burn FURB tokens
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.furb_mint.to_account_info(),
                from: ctx.accounts.user_furb_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    // Mint FURA tokens
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.fura_mint.to_account_info(),
                to: ctx.accounts.user_fura_account.to_account_info(),
                authority: ctx.accounts.conversion_vault.to_account_info(),
            },
        ),
        fura_amount,
    )?;

    // Update vault balances
    conversion_vault.furb_balance += amount;
    conversion_vault.fura_balance -= fura_amount;

    Ok(())
}

#[derive(Accounts)]
pub struct TokenConversion<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::mint = furb_mint,
        token::authority = user
    )]
    pub user_furb_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = fura_mint,
        token::authority = user
    )]
    pub user_fura_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub furb_mint: Account<'info, Mint>,

    #[account(mut)]
    pub fura_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"conversion_vault"],
        bump
    )]
    pub conversion_vault: Account<'info, TokenConversionVault>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid conversion amount")]
    InvalidConversionAmount,
}
