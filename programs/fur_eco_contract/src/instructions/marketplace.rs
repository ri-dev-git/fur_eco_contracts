use crate::state::marketplace::{ItemType, MarketplaceListing};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

pub fn create_listing(
    ctx: Context<CreateMarketplaceListing>,
    price: u64,
    item_type: ItemType,
) -> Result<()> {
    let listing = &mut ctx.accounts.listing;

    // Transfer item to marketplace escrow
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.seller_item_account.to_account_info(),
                to: ctx.accounts.marketplace_escrow.to_account_info(),
                authority: ctx.accounts.seller.to_account_info(),
            },
        ),
        1, // Assuming NFT transfer (1 token)
    )?;

    // Create listing
    listing.seller = ctx.accounts.seller.key();
    listing.item_mint = ctx.accounts.item_mint.key();
    listing.price = price;
    listing.item_type = item_type;
    listing.is_sold = false;

    Ok(())
}

pub fn purchase_listing(ctx: Context<PurchaseFromMarketplace>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;

    // Ensure listing is not sold
    require!(!listing.is_sold, ErrorCode::ListingAlreadySold);

    // Transfer payment
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.buyer_payment_account.to_account_info(),
                to: ctx.accounts.seller_payment_account.to_account_info(),
                authority: ctx.accounts.buyer.to_account_info(),
            },
        ),
        listing.price,
    )?;

    // Transfer item from escrow to buyer
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.marketplace_escrow.to_account_info(),
                to: ctx.accounts.buyer_item_account.to_account_info(),
                authority: ctx.accounts.listing.to_account_info(),
            },
        ),
        1, // Assuming NFT transfer (1 token)
    )?;

    // Mark listing as sold
    listing.is_sold = true;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateMarketplaceListing<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        init,
        payer = seller,
        space = 8 + MarketplaceListing::LEN
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(
        mut,
        token::mint = item_mint,
        token::authority = seller
    )]
    pub seller_item_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub item_mint: Account<'info, Mint>,

    #[account(mut)]
    pub marketplace_escrow: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PurchaseFromMarketplace<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(
        mut,
        has_one = seller,
        close = seller
    )]
    pub listing: Account<'info, MarketplaceListing>,

    #[account(
        mut,
        token::mint = item_mint,
        token::authority = buyer
    )]
    pub buyer_item_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = payment_mint,
        token::authority = buyer
    )]
    pub buyer_payment_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = payment_mint,
        token::authority = seller
    )]
    pub seller_payment_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub item_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payment_mint: Account<'info, Mint>,

    #[account(mut)]
    pub marketplace_escrow: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
