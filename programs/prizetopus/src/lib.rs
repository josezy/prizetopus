use anchor_lang::prelude::*;
use instructions::*;
use state::*;

pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
 
#[program]
pub mod prizetopus {
    use super::*;

    pub fn init_prize_pool(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn init_prize_distribution(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn init_tournament(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn join_tournament(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn start_tournament(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn eliminate_player(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    
    pub fn claim_prize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    

}

#[derive(Accounts)]
pub struct Initialize {}
