use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct FurFT {
    pub owner: Pubkey,
    pub item_type: PetType,
    pub rarity: Rarity,
    pub fusion_count: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

pub fn create_furft(ctx: Context<CreateFurFT>, item_type: ItemType, rarity: Rarity) -> Result<()> {
    let furft = &mut ctx.accounts.furft;

    furft.owner = ctx.accounts.user.key();
    furft.item_type = item_type;
    furft.rarity = rarity;
    furft.fusion_count = 0;

    // Mint FurFT token
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.furft_mint.to_account_info(),
                to: ctx.accounts.user_furft_account.to_account_info(),
                authority: ctx.accounts.minter.to_account_info(),
            },
        ),
        1,
    )?;

    Ok(())
}

pub fn fuse_furfts(ctx: Context<FuseFurFTs>) -> Result<()> {
    let base_furft = &mut ctx.accounts.base_furft;
    let fusion_furft = &mut ctx.accounts.fusion_furft;

    // Check fusion compatibility
    require!(
        base_furft.item_type == fusion_furft.item_type,
        ErrorCode::IncompatibleFusion
    );

    // Determine new rarity
    let new_rarity = match (base_furft.rarity, fusion_furft.rarity) {
        (Rarity::Common, Rarity::Common) => Rarity::Rare,
        (Rarity::Rare, Rarity::Rare) => Rarity::Epic,
        (Rarity::Epic, Rarity::Epic) => Rarity::Legendary,
        _ => return Err(ErrorCode::MaxRarityReached.into()),
    };

    // Burn fusion FurFT
    token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: ctx.accounts.fusion_furft_mint.to_account_info(),
                from: ctx.accounts.user_fusion_furft_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        1,
    )?;

    // Update base FurFT
    base_furft.rarity = new_rarity;
    base_furft.fusion_count += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateFurFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 1 + 1 + 1
    )]
    pub furft: Account<'info, FurFT>,

    #[account(
        mut,
        token::mint = furft_mint,
        token::authority = user
    )]
    pub user_furft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub furft_mint: Account<'info, Mint>,

    pub minter: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FuseFurFTs<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        has_one = owner
    )]
    pub base_furft: Account<'info, FurFT>,

    #[account(
        mut,
        has_one = owner
    )]
    pub fusion_furft: Account<'info, FurFT>,

    #[account(
        mut,
        token::mint = base_furft_mint,
        token::authority = user
    )]
    pub user_base_furft_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = fusion_furft_mint,
        token::authority = user
    )]
    pub user_fusion_furft_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub base_furft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub fusion_furft_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Incompatible FurFT fusion")]
    IncompatibleFusion,
    #[msg("Maximum rarity reached")]
    MaxRarityReached,
}
