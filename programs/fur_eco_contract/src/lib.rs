use anchor_lang::prelude::*;

declare_id!("7VB9bByo3zkshwzUEN4DBq9EQKMedSU81gdFyGVQgAHW");

#[program]
pub mod fur_eco_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
