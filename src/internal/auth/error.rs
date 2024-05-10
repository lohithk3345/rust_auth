use axum::{http::StatusCode, response::IntoResponse};
use serde_json::json;

pub type Result<T> = core::result::Result<T, TokenError>;

#[derive(Debug, Clone)]
pub enum TokenError {
    NotFound,
    InvalidSignature,
    InvalidToken,
    TokenExpired,
    Unknown,
}
