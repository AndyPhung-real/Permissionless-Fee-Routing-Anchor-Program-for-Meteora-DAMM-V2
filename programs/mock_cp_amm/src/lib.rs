// BEGIN programs/mock_cp_amm/src/lib.rs
#![deny(unsafe_code)]
use anchor_lang::prelude::*;

declare_id!("MockCpAmm1111111111111111111111111111111");

#[program]
pub mod mock_cp_amm {
    use super::*;

    pub fn init_pool(_ctx: Context<InitPool>, quote: Pubkey, base: Pubkey) -> Result<()> {
        let pool = &mut _ctx.accounts.pool;
        pool.quote_mint = quote;
        pool.base_mint  = base;
        Ok(())
    }

    pub fn create_position(_ctx: Context<CreatePosition>) -> Result<()> {
        let pos = &mut _ctx.accounts.position;
        pos.pool       = _ctx.accounts.pool.key();
        pos.owner      = _ctx.accounts.owner.key();
        pos.quote_fee  = 0;
        pos.base_fee   = 0;
        Ok(())
    }

    pub fn add_quote_fee(_ctx: Context<UpdateFee>, amount: u64) -> Result<()> {
        let pos = &mut _ctx.accounts.position;
        pos.quote_fee = pos.quote_fee.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn add_base_fee(_ctx: Context<UpdateFee>, amount: u64) -> Result<()> {
        let pos = &mut _ctx.accounts.position;
        pos.base_fee = pos.base_fee.checked_add(amount).unwrap();
        Ok(())
    }

    pub fn claim_fees(_ctx: Context<UpdateFee>) -> Result<()> {
        let pos = &mut _ctx.accounts.position;
        pos.quote_fee = 0;
        pos.base_fee  = 0;
        Ok(())
    }
}

/* ------------------------- accounts & state -------------------------- */

#[account]
pub struct Pool {
    pub quote_mint: Pubkey,
    pub base_mint:  Pubkey,
}

#[account]
pub struct Position {
    pub pool:      Pubkey,
    pub owner:     Pubkey,
    pub quote_fee: u64,
    pub base_fee:  u64,
}

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 32)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePosition<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(init, payer = payer, space = 8 + 32 + 32 + 8 + 8)]
    pub position: Account<'info, Position>,
    /// CHECK: PDA that owns the position
    pub owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut)]
    pub position: Account<'info, Position>,
}
// END programs/mock_cp_amm/src/lib.rs
