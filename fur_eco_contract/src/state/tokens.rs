use anchor_lang::prelude::*;

#[account]
pub struct TokenConversionVault {
    pub furb_balance: u64,
    pub fura_balance: u64,
    pub conversion_rate: u16, // 5000 FURB to 5 FURA
}

impl TokenConversionVault {
    pub const LEN: usize = 8 + // discriminator
        8 + // FURB balance
        8 + // FURA balance
        2; // conversion rate
}
