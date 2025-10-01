// BEGIN programs/fee_router/src/instructions/crank.rs
use anchor_lang::prelude::*;
use crate::{contexts::*, state::*, errors::RouterError};

pub fn handler(ctx: Context<CrankPage>, _page_len: u32) -> Result<()> {
    let clock = Clock::get()?;
    let today = unix_day(clock.unix_timestamp);

    /* 1. 24h gate -------------------------------------------------------- */
    if ctx.accounts.progress_pda.day != today {
        // first crank of the day ─ reset the PDA
        ctx.accounts.progress_pda.day         = today;
        ctx.accounts.progress_pda.distributed = 0;
        ctx.accounts.progress_pda.carry_quote = 0;
        ctx.accounts.progress_pda.cursor      = 0;
    }

    /* 2. Read fees on the position -------------------------------------- */
    let pos: mock_cp_amm::Position = Account::try_from(&ctx.accounts.honorary_position)?;
    require!(pos.base_fee == 0, RouterError::BaseFeeDetected);

    let quote_claim = pos.quote_fee;
    require!(quote_claim > 0, RouterError::BaseFeeRisk); // re-use as “nothing to do”

    /* 3. Reset fees (CPI into mock_cp_amm) ------------------------------ */
    {
        let cpi_program = ctx.accounts.cp_amm_program.to_account_info();
        let cpi_accounts = mock_cp_amm::accounts::UpdateFee {
            position: ctx.accounts.honorary_position.to_account_info(),
        };
        mock_cp_amm::claim_fees(CpiContext::new(cpi_program, cpi_accounts))?;
    }

    /* 4. Calculate investor share --------------------------------------- */
    let policy = &ctx.accounts.policy_pda;
    let mut locked_total: u64 = 0;
    let mut stream_accounts: Vec<Account<mock_streamflow::Stream>> = Vec::new();

    // remaining_accounts layout = [stream_0, stream_1, …] only
    for acc in ctx.remaining_accounts.iter() {
        let st: Account<mock_streamflow::Stream> = Account::try_from(acc)?;
        locked_total = locked_total.checked_add(st.locked).ok_or(RouterError::MathOverflow)?;
        stream_accounts.push(st);
    }

    let f_locked_bps = if policy.y0 == 0 { 0 }
        else { ((locked_total as u128 * 10_000) / policy.y0 as u128) as u16 };
    let eligible_bps = std::cmp::min(policy.investor_fee_share_bps, f_locked_bps);
    let investor_pot = quote_claim * (eligible_bps as u64) / 10_000;
    let creator_part = quote_claim - investor_pot;

    /* 5. Distribute pro-rata to streams --------------------------------- */
    let mut distributed_now = 0u64;
    for (idx, mut st) in stream_accounts.into_iter().enumerate() {
        // weight_i = locked_i / locked_total
        if locked_total == 0 { break; }
        let payout = investor_pot * st.locked / locked_total;
        if payout < policy.min_payout_lamports { continue; }
        st.paid_quote = st.paid_quote.checked_add(payout).unwrap();
        distributed_now += payout;
        st.exit(&crate::ID)?;                           // write-back
        emit!(InvestorPaid { index: idx as u32, amount: payout });
    }

    /* 6. Update progress & send remainder to creator -------------------- */
    ctx.accounts.progress_pda.distributed =
        ctx.accounts.progress_pda.distributed.checked_add(distributed_now).unwrap();
    ctx.accounts.progress_pda.carry_quote =
        ctx.accounts.progress_pda.carry_quote.checked_add(creator_part).unwrap();

    emit!(CreatorPayoutDayClosed { remainder: creator_part });
    Ok(())
}

#[event]
pub struct InvestorPaid {
    pub index:  u32,
    pub amount: u64,
}

#[event]
pub struct CreatorPayoutDayClosed {
    pub remainder: u64,
}
// END programs/fee_router/src/instructions/crank.rs
