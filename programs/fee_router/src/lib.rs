// BEGIN programs/fee_router/src/lib.rs
#![deny(unsafe_code)]
use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;
pub mod instructions;
pub mod errors;

use instructions::*;

declare_id!("FeeRouTer11111111111111111111111111111111");

#[program]
pub mod fee_router {
    use super::*;

    /// Work package A – create the honorary LP position (quote only).
    pub fn init_honorary_position(ctx: Context<InitHonoraryPosition>) -> Result<()> {
        init_honorary_position::handler(ctx)
    }

    /// Work package B – permissionless once-per-day crank (with pagination).
    pub fn crank_page(ctx: Context<CrankPage>, page_len: u32) -> Result<()> {
        crank::handler(ctx, page_len)
    }
}
// END programs/fee_router/src/lib.rs
