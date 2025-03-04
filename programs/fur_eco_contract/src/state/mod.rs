use anchor_lang::prelude::*;

pub mod furft;
pub mod marketplace;
pub mod pet;
pub mod tokens;

pub use furft::{FurFT, Rarity};
pub use marketplace::{ItemType, MarketplaceListing};
pub use pet::{Pet, PetType};
pub use tokens::TokenConversionVault;

// Shared state or global configuration
#[account]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub furb_total_supply: u64,
    pub fura_total_supply: u64,
    pub protocol_fee_rate: u16,   // basis points
    pub staking_reward_rate: u16, // basis points per month
}

impl GlobalConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // admin
        8 + // FURB total supply
        8 + // FURA total supply
        2 + // protocol fee rate
        2; // staking reward rate
}

// Governance account to track community proposals
#[account]
pub struct GovernanceProposal {
    pub proposer: Pubkey,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub executed: bool,
    pub proposal_type: ProposalType,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProposalType {
    SupplyChange,
    ProtocolFeeUpdate,
    StakingRewardUpdate,
    Other,
}

impl GovernanceProposal {
    pub const LEN: usize = 8 + // discriminator
        32 + // proposer
        (4 + 256) + // description (max 256 chars)
        8 + // votes for
        8 + // votes against
        8 + // start time
        8 + // end time
        1 + // executed
        1; // proposal type
}
