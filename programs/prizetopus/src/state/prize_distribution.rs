use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct PrizeDistribution {
    pub prizes: [u64; 10] // [50, 30, 20, 0, 0, 0, 0, 0, 0, 0]
}