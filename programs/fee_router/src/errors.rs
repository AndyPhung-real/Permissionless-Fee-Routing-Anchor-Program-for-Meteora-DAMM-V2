// BEGIN programs/fee_router/src/errors.rs
use anchor_lang::prelude::*;

#[error_code]
pub enum RouterError {
    #[msg("Honorary position would accrue base fees – abort")]
    BaseFeeRisk,
    #[msg("Non-zero base fees detected, distribution cancelled")]
    BaseFeeDetected,
    #[msg("Crank can run only once per 24 h")]
    Gate24h,
    #[msg("Math overflow")]
    MathOverflow,
}
// END programs/fee_router/src/errors.rs
