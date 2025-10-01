// BEGIN programs/fee_router/src/state.rs
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Policy {
    pub y0: u64,                     // total investor mint @ TGE
    pub investor_fee_share_bps: u16, // max basis-points sent to investors
    pub daily_cap_quote: u64,        // 0 = no cap
    pub min_payout_lamports: u64,    // dust threshold
    pub bump: u8,
}

// One Progress account per calendar day (unix_day = ts / 86_400)
#[account]
#[derive(Default)]
pub struct Progress {
    pub day: u32,
    pub distributed: u64, // quote sent to investors so far today
    pub carry_quote: u64, // quote held for tomorrow (dust + cap remainder)
    pub cursor: u32,      // pagination index
    pub bump: u8,
}

pub fn unix_day(ts: i64) -> u32 {
    (ts / 86_400) as u32
}
// END programs/fee_router/src/state.rs
