use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PetType {
    Egg,
    Baby,
    Teen,
    Adult,
}

#[account]
pub struct Pet {
    pub owner: Pubkey,
    pub pet_type: PetType,
    pub level: u8,
    pub experience: u64,
    pub last_evolution_timestamp: i64,
}

impl Pet {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        1 + // pet_type
        1 + // level
        8 + // experience
        8; // last_evolution_timestamp
}
