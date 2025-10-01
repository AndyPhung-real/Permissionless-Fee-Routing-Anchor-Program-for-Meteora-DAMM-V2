// BEGIN programs/fee_router/src/contexts.rs
use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct InitHonoraryPosition<'info> {
    // ---- signer paying for rent & tx fees
    #[account(mut)]
    pub payer: Signer<'info>,

    // ---- PDA that will own the LP NFT
    /// CHECK: only used as PDA seed, validated in handler
    #[account(mut)]
    pub investor_fee_pos_owner_pda: UncheckedAccount<'info>,

    // ---- placeholder for the new DAMM position account
    /// CHECK: created via CPI to cp-amm
    #[account(mut)]
    pub new_position: UncheckedAccount<'info>,

    // ---- remaining accounts (pool, spl-token, system, rent, etc.)
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: CP-AMM program
    pub cp_amm_program: UncheckedAccount<'info>,
    /// CHECK: CP-AMM pool
    pub cp_amm_pool: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CrankPage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // Progress PDA for *today*
    #[account(
        mut,
        seeds = [b"progress", &state::unix_day(clock::Clock::get()?.unix_timestamp).to_le_bytes()],
        bump,
    )]
    pub progress_pda: Account<'info, Progress>,

    // Static policy config
    #[account(seeds = [b"policy"], bump)]
    pub policy_pda: Account<'info, Policy>,

    // Honorary LP position owner & the position itself
    /// CHECK:
    pub investor_fee_pos_owner_pda: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    pub honorary_position: UncheckedAccount<'info>,

    // Quote treasury owned by program
    #[account(mut)]
    pub quote_treasury_ata: AccountInfo<'info>,

    // Creator’s ATA (remainder routed here)
    #[account(mut)]
    pub creator_quote_ata: AccountInfo<'info>,

    // Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK:
    pub cp_amm_program: UncheckedAccount<'info>,
    /// CHECK:
    pub streamflow_program: UncheckedAccount<'info>,

    // Remaining accounts: pool, vaults, investor pages, etc.
    /// CHECK:
    #[account(mut)]
    pub remaining_accounts: Vec<AccountInfo<'info>>,
    /* helper re-export for crank.rs */
pub use crate::state::unix_day;
}
// END programs/fee_router/src/contexts.rs
