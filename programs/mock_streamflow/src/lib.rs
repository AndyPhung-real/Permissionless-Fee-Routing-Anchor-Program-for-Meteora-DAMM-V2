// BEGIN programs/mock_streamflow/src/lib.rs
#![deny(unsafe_code)]
use anchor_lang::prelude::*;

declare_id!("MockStrm11111111111111111111111111111111");

#[program]
pub mod mock_streamflow {
    use super::*;

    pub fn init_stream(ctx: Context<InitStream>, locked: u64) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        stream.beneficiary = ctx.accounts.beneficiary.key();
        stream.locked      = locked;
        stream.paid_quote  = 0;
        Ok(())
    }

    pub fn set_locked(ctx: Context<SetLocked>, new_locked: u64) -> Result<()> {
        ctx.accounts.stream.locked = new_locked;
        Ok(())
    }
}

#[account]
pub struct Stream {
    pub beneficiary: Pubkey,
    pub locked:      u64,
    pub paid_quote:  u64,   // how much quote received so far (test-only)
}

#[derive(Accounts)]
pub struct InitStream<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8 + 8)]
    pub stream: Account<'info, Stream>,
    /// CHECK:
    pub beneficiary: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetLocked<'info> {
    #[account(mut)]
    pub stream: Account<'info, Stream>,
}
// END programs/mock_streamflow/src/lib.rs
