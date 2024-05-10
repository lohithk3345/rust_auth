use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

use crate::internal::{auth::error::TokenError, data::error::DataError};

pub type Result<T> = core::result::Result<T, ServiceError>;

#[derive(Debug, Clone)]
pub enum ServiceError {
    TokenNotFound,
    TokenInvalidSignature,
    TokenInvalid,
    TokenExpired,
    Duplicate,
    NotFound,
    UserPasswordUnauthorized,
    Unknown,
}

// impl From<DataError> for ServiceError {
//     fn from(value: DataError) -> Self {
//         match value {
//             DataError::Duplicate => ServiceError::Duplicate,
//             DataError::NotFound => ServiceError::NotFound,
//             DataError::Unknown => ServiceError::Unknown,
//             DataError::InsertFailed => ServiceError::Unknown,
//             // _ => Self::Unknown
//         }
//     }
// }

impl ServiceError {
    // pub fn from_self<T>(err: T) -> Self {
    //     Self { err}
    // }

    pub fn from_token_error(value: TokenError) -> Self {
        match value {
            TokenError::InvalidSignature => ServiceError::TokenInvalidSignature,
            TokenError::InvalidToken => ServiceError::TokenInvalid,
            TokenError::TokenExpired => ServiceError::TokenExpired,
            TokenError::NotFound => ServiceError::TokenNotFound,
            TokenError::Unknown => ServiceError::Unknown,
            // _ => Self::Unknown
        }
    }

    pub fn from_data_error(value: DataError) -> Self {
        dbg!(&value);
        match value {
            DataError::Duplicate => ServiceError::Duplicate,
            DataError::NotFound => ServiceError::NotFound,
            DataError::InsertFailed => ServiceError::Unknown,
            DataError::DeleteFailed => ServiceError::Unknown,
            DataError::Unknown => ServiceError::Unknown,
        }
    }

    // pub fn get_token_error_response(value: TokenError) -> impl IntoResponse {
    //     match value {
    //         TokenError::InvalidSignature => (
    //             StatusCode::UNAUTHORIZED,
    //             Json(json!({"error": "User Unauthorized"})),
    //         )
    //             .into_response(),
    //         TokenError::InvalidToken => (
    //             StatusCode::UNAUTHORIZED,
    //             Json(json!({"error": "Unauthorized. Invalid Token"})),
    //         )
    //             .into_response(),
    //         TokenError::TokenExpired => (
    //             StatusCode::UNAUTHORIZED,
    //             Json(json!({"error": "Unauthorized. Token Expired"})),
    //         )
    //             .into_response(),
    //         TokenError::Unknown => (
    //             StatusCode::INTERNAL_SERVER_ERROR,
    //             Json(json!({"status": "Internal Server Error"})),
    //         )
    //             .into_response(),
    //     }
    // }

    pub fn get_user_error_response(self) -> impl IntoResponse {
        // move |body|
        match self {
            ServiceError::Duplicate => (
                StatusCode::CONFLICT,
                Json(json!({"error": "User Already Exists"})),
            )
                .into_response(),
            ServiceError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User Not Found"})),
            )
                .into_response(),
            ServiceError::TokenInvalidSignature => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "User Unauthorized"})),
            )
                .into_response(),
            ServiceError::TokenInvalid => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized. Invalid Token"})),
            )
                .into_response(),
            ServiceError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized. Token Expired"})),
            )
                .into_response(),
            ServiceError::TokenNotFound => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized. Token Not Found"})),
            )
                .into_response(),
            Self::UserPasswordUnauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized. Username Or Password Is Incorrect"})),
            )
                .into_response(),
            ServiceError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "Internal Server Error"})),
            )
                .into_response(),
        }
    }
}

impl core::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ServiceError {}
