use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct InitPrizeDistribution<'info> {
    #[account(
        init,
        seeds = [b"prize_distribution".as_ref()],
        bump = bump,
        payer = payer,
        space = 8
    )]
    pub prize_distribution: Account<'info, PrizeDistribution>,
    #[account(signer)]
    pub payer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
}