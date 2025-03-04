use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ItemType {
    Pet,
    Land,
    Accessory,
}

#[account]
pub struct MarketplaceListing {
    pub seller: Pubkey,
    pub item_mint: Pubkey,
    pub price: u64,
    pub item_type: ItemType,
    pub is_sold: bool,
}

impl MarketplaceListing {
    pub const LEN: usize = 8 + // discriminator
        32 + // seller
        32 + // item mint
        8 +  // price
        1 +  // item type
        1; // is sold
}
