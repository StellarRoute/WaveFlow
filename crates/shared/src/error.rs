// Shared error types used across gateway, API, and future SDK clients.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WaveFlowError {
    #[error("configuration error: {0}")]
    Config(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("webhook error: {0}")]
    Webhook(String),

    #[error("chain error: {0}")]
    Chain(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type WaveFlowResult<T> = Result<T, WaveFlowError>;

impl WaveFlowError {
    pub fn http_status_code(&self) -> u16 {
        match self {
            Self::Unauthorized(_) => 401,
            Self::Validation(_) | Self::Webhook(_) => 400,
            Self::NotFound(_) => 404,
            Self::Conflict(_) => 409,
            Self::Config(_) | Self::Database(_) | Self::Chain(_) | Self::Internal(_) => 500,
        }
    }
}
