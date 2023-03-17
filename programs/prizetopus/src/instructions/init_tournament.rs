use anchor_lang::prelude::*;

use crate::state::*;

pub const PREFIX: &str = "virtue_poker";

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct InitTournament<'info> {
    #[account(init, payer = payer, space = 8 + std::mem::size_of::<Tournament>())]
    pub tournament: Account<'info, Tournament>,

    pub tournament_authority: Account<'info>,

    // Prize pool
    #[account(mut, seeds=[PREFIX.as_bytes(), tournament.key().as_ref(), b"prize_pool".as_ref()], bump)]
    pub prize_pool: Account<'info>,
    #[account(
        init,
        seeds = [ PREFIX.as_bytes(), tournament.key(),b"prize_distribution".as_ref()],
        bump = bump,
        payer = payer,
        space = 8 + std::mem::size_of::<PrizeDistribution>()
    )]
    pub prize_distribution: Account<'info, PrizeDistribution>,
}
