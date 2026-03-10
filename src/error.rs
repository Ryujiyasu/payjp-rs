//! Error types for the PAY.JP SDK.

use serde::Deserialize;

/// Errors that can occur when interacting with the PAY.JP API.
#[derive(Debug, thiserror::Error)]
pub enum PayjpError {
    #[error("API error: {0}")]
    Api(ApiError),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Webhook verification failed: {0}")]
    WebhookVerification(String),
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub message: String,
    pub code: Option<String>,
    pub r#type: String,
    pub status: u16,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.status, self.message)
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorResponse {
    pub error: ApiError,
}

pub type Result<T> = std::result::Result<T, PayjpError>;
