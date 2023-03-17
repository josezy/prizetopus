use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct Tournament {
    pub authority: Pubkey,
    pub prize_pool: Pubkey,
    pub prize_distribution: Pubkey,
    pub buy_in: u64,
    pub max_players: u64,
    pub started: bool,
    pub timeout: u64,
}