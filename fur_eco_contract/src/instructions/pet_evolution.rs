use anchor_lang::prelude::*;
use crate::state::pet::{Pet, PetType};

pub fn initialize_pet(ctx: Context<InitializePet>, pet_type: PetType) -> Result<()> {
    let pet = &mut ctx.accounts.pet;
    pet.owner = ctx.accounts.user.key();
    pet.pet_type = pet_type;
    pet.level = 1;
    pet.experience = 0;
    pet.last_evolution_timestamp = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn evolve_pet(ctx: Context<EvolvePet>) -> Result<()> {
    let pet = &mut ctx.accounts.pet;
    let current_time = Clock::get()?.unix_timestamp;

    // Evolution logic
    match pet.pet_type {
        PetType::Egg => {
            require!(current_time - pet.last_evolution_timestamp >= 86400, ErrorCode::EvolveTimeLimitNotMet);
            pet.pet_type = PetType::Baby;
        },
        PetType::Baby => {
            require!(pet.experience >= 100, ErrorCode::InsufficientExperience);
            pet.pet_type = PetType::Teen;
        },
        PetType::Teen => {
            require!(pet.experience >= 500, ErrorCode::InsufficientExperience);
            pet.pet_type = PetType::Adult;
        },
        PetType::Adult => {
            return Err(ErrorCode::MaxEvolutionReached.into());
        }
    }

    pet.level += 1;
    pet.last_evolution_timestamp = current_time;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializePet<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + Pet::LEN
    )]
    pub pet: Account<'info, Pet>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvolvePet<'info> {
    #[account(
        mut,
        has_one = owner
    )]
    pub pet: Account<'info, Pet>,
    
    pub owner: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Evolution time limit not met")]
    EvolveTimeLimitNotMet,
    #[msg("Insufficient experience to evolve")]
    InsufficientExperience,
    #[msg("Pet has reached maximum evolution stage")]
    MaxEvolutionReached,
}