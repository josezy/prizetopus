use anchor_lang::prelude::*;

use crate::state::*;

const PREFIX: &str = "virtue_poker";

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct JoinTournament<'info> {

    #[account(mut)]
    pub tournament: Account<'info, Tournament>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [ 
            PREFIX.as_bytes(), 
            tournament.key().as_ref(),
            b"player_state".as_ref()
            ],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<PlayerState>()
    )]
    pub player_state: Account<'info, PlayerState>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<JoinTournament>,
    player_state_bump: u8,
) -> Result<()> {

    Ok(())
}