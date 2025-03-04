use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[account]
pub struct StakingAccount {
    pub owner: Pubkey,
    pub furb_staked: u64,
    pub stake_start_time: i64,
    pub accumulated_rewards: u64,
}

impl StakingAccount {
    pub const LEN: usize = 8 + // discriminator
        32 + // owner
        8 +  // FURB staked
        8 +  // stake start time
        8; // accumulated rewards
}

pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Transfer FURB tokens to staking vault
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.user_furb_account.to_account_info(),
                to: ctx.accounts.staking_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    // Update staking account
    staking_account.owner = ctx.accounts.user.key();
    staking_account.furb_staked += amount;
    staking_account.stake_start_time = current_time;

    Ok(())
}

pub fn claim_staking_rewards(ctx: Context<ClaimStakingRewards>) -> Result<()> {
    let staking_account = &mut ctx.accounts.staking_account;
    let current_time = Clock::get()?.unix_timestamp;

    // Calculate rewards (0.1% per 30 days)
    let stake_duration = current_time - staking_account.stake_start_time;
    let months_staked = stake_duration / (30 * 24 * 60 * 60); // seconds in 30 days
    let reward_rate_per_month = 0.001; // 0.1%

    let total_rewards = (staking_account.furb_staked as f64
        * (months_staked as f64 * reward_rate_per_month)) as u64;

    // Mint rewards to user
    token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.fura_mint.to_account_info(),
                to: ctx.accounts.user_fura_account.to_account_info(),
                authority: ctx.accounts.staking_vault.to_account_info(),
            },
        ),
        total_rewards,
    )?;

    // Reset staking account
    staking_account.accumulated_rewards += total_rewards;
    staking_account.stake_start_time = current_time;

    Ok(())
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        token::mint = furb_mint,
        token::authority = user
    )]
    pub user_furb_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + StakingAccount::LEN,
        seeds = [b"staking", user.key().as_ref()],
        bump
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub furb_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimStakingRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"staking", user.key().as_ref()],
        bump,
        has_one = owner
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
        mut,
        token::mint = fura_mint,
        token::authority = user
    )]
    pub user_fura_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fura_mint: Account<'info, Mint>,

    #[account(mut)]
    pub staking_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}
