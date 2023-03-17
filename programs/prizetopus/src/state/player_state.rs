use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct PlayerState {
    pub eliminated: bool,
}