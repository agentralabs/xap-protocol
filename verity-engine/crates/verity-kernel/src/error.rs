use crate::Currency;

#[derive(Debug, thiserror::Error)]
pub enum VerityError {
    #[error("Invalid ID format: {0}")]
    InvalidIdFormat(String),

    #[error("Basis points out of range: {0} (must be 0-10000)")]
    BasisPointsOutOfRange(u16),

    #[error("Shares do not sum to 10000: got {0}")]
    SharesSumInvalid(u32),

    #[error("Currency mismatch: cannot operate on {0:?} and {1:?}")]
    CurrencyMismatch(Currency, Currency),

    #[error("Arithmetic overflow")]
    ArithmeticOverflow,

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Invalid transition from {from:?} to {to:?}: {reason}")]
    InvalidTransition {
        from: String,
        to: String,
        reason: String,
    },

    #[error("Chain integrity error: {0}")]
    ChainIntegrityError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),
}
