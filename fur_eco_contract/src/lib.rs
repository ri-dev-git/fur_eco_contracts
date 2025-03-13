use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::furb::*;
declare_id!("FurEco1111111111111111111111111111111111111");

#[program]
pub mod fur_ecosystem {
    use super::*;

    // pub fn initialize_mint(ctx: Context<>) -> Result<()> {
    //     furb::initialize(ctx);
    // }

    // pub fn initialize_pet(ctx: Context<InitializePet>, pet_type: PetType) -> Result<()> {
    //     instructions::pet_evolution::initialize_pet(ctx, pet_type)
    // }

    // pub fn evolve_pet(ctx: Context<EvolvePet>) -> Result<()> {
    //     instructions::pet_evolution::evolve_pet(ctx)
    // }

    // pub fn convert_tokens(ctx: Context<TokenConversion>, amount: u64) -> Result<()> {
    //     instructions::token_conversion::convert_tokens(ctx, amount)
    // }

    // pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
    //     instructions::staking::stake_tokens(ctx, amount)
    // }

    // pub fn create_marketplace_listing(
    //     ctx: Context<CreateMarketplaceListing>,
    //     price: u64,
    //     item_type: ItemType,
    // ) -> Result<()> {
    //     instructions::marketplace::create_listing(ctx, price, item_type)
    // }

    // pub fn purchase_from_marketplace(ctx: Context<PurchaseFromMarketplace>) -> Result<()> {
    //     instructions::marketplace::purchase_listing(ctx)
    // }
}
