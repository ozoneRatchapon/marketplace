use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name too long")]
    NameTooLong,
    #[msg("Please check for underflow and overflow")]
    ArithmeticOverflow,
    #[msg("Fee% 0 to 1000 (0-100%)")]
    InvalidFee,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Invalid metadata")]
    InvalidMetadata,
}
