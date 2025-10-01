// BEGIN programs/fee_router/src/instructions/crank.rs
use anchor_lang::prelude::*;
use crate::{contexts::*, errors::RouterError, state::*};

pub fn handler(ctx: Context<CrankPage>, _page_len: u32) -> Result<()> {
    // Stub implementation:
    // The real version would:
    //   • claim fees from cp-amm,
    //   • compute investor share based on still-locked balances,
    //   • stream quote tokens,
    //   • update Progress PDA with cursor / carry-over.
    msg!("crank_page: stub (replace with real logic)");
    Ok(())
}
// END programs/fee_router/src/instructions/crank.rs
