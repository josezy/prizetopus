use anchor_lang::prelude::*;
use instructions::*;
// use state::*;

pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
 
#[program]
pub mod prizetopus {
    use super::*;

    pub fn init_tournament(
        ctx: Context<InitTournament>,
        prize_pool_bump: u8,
        prize_distribution_bump: u8,
        buy_in: u64,
        max_players: u64,
        timeout: u64,
    ) -> Result<()> {
        instructions::init_tournament::handler(ctx, prize_pool_bump, prize_distribution_bump, buy_in, max_players, timeout)
    }

    pub fn join_tournament(
        ctx: Context<JoinTournament>,
        player_state_bump: u8,
    ) -> Result<()> {
        instructions::join_tournament::handler(ctx, player_state_bump)
    }
    
    pub fn start_tournament(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn eliminate_player(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn claim_prize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
