// BEGIN programs/fee_router/src/instructions/init_honorary_position.rs
use anchor_lang::prelude::*;
use crate::contexts::*;
use crate::errors::RouterError;

pub fn handler(ctx: Context<InitHonoraryPosition>) -> Result<()> {
    // CPI into mock_cp_amm to create a fresh position owned by our PDA.
    let cpi_program = ctx.accounts.cp_amm_program.to_account_info();
    let cpi_accounts = mock_cp_amm::accounts::CreatePosition {
        pool:      ctx.accounts.cp_amm_pool.to_account_info(),
        position:  ctx.accounts.new_position.to_account_info(),
        owner:     ctx.accounts.investor_fee_pos_owner_pda.to_account_info(),
        payer:     ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    mock_cp_amm::create_position(cpi_ctx)?;

    emit!(HonoraryPositionInitialized {
        position: ctx.accounts.new_position.key(),
    });
    Ok(())
}

#[event]
pub struct HonoraryPositionInitialized {
    pub position: Pubkey,
}
// END programs/fee_router/src/instructions/init_honorary_position.rs
