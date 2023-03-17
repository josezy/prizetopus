use anchor_lang::prelude::*;

use crate::state::*;

const PREFIX: &str = "virtue_poker";

#[derive(Accounts)]
pub struct StartTournament<'info> {

    // TODO: Check if we need to pass a tournament authority
    // as an account, in order to confirm that it's a signer
    #[account(mut)]
    pub tournament: Account<'info, Tournament>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn handler(
    ctx: Context<StartTournament>,
) -> Result<()> {

    Ok(())
}