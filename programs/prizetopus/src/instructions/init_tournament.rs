use anchor_lang::prelude::*;

use crate::state::*;

pub const PREFIX: &str = "virtue_poker";

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitTournament<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Tournament>())]
    pub tournament: Account<'info, Tournament>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub tournament_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    // Prize pool
    #[account(
        init, 
        seeds=[
            PREFIX.as_bytes(), 
            tournament.key().as_ref(), 
            b"prize_pool".as_ref()
            ], 
        bump,
        payer = payer,
        space = 8
    )]

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub prize_pool: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [ 
            PREFIX.as_bytes(), 
            tournament.key().as_ref(),
            b"prize_distribution".as_ref()
            ],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<PrizeDistribution>()
    )]
    pub prize_distribution: Account<'info, PrizeDistribution>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    _ctx: Context<InitTournament>,
    _prize_pool_bump: u8,
    _prize_distribution_bump: u8,
    _buy_in: u64,
    _max_players: u64,
    _timeout: u64,
) -> Result<()> {
    msg!("Got into handler");
    Ok(())
}