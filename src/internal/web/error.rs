use std::any::TypeId;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::internal::{
    auth::error::TokenError, data::error::DataError, services::error::ServiceError,
};

pub type Result<T> = core::result::Result<T, Response>;

#[derive(Debug, Clone)]
pub enum ApiError {
    Service(ServiceError),
    Token(TokenError),
    Data(DataError),
}

impl core::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Service(err) => Self::handle_service_error(&err).into_response(),
            ApiError::Token(_) => todo!(),
            ApiError::Data(err) => {
                dbg!(err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

impl ApiError {
    pub fn handle_service_error(err: &ServiceError) -> impl IntoResponse {
        match err {
            ServiceError::TokenNotFound => todo!(),
            ServiceError::TokenInvalidSignature => todo!(),
            ServiceError::TokenInvalid => todo!(),
            ServiceError::TokenExpired => todo!(),
            ServiceError::Duplicate => todo!(),
            ServiceError::NotFound => todo!(),
            ServiceError::UserPasswordUnauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Internal Server Error"})),
            )
                .into_response(),
            ServiceError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal Server Error"})),
            )
                .into_response(),
        }
    }

    pub fn handle_data_error(err: &DataError) -> impl IntoResponse {
        match err {
            DataError::Duplicate => todo!(),
            DataError::NotFound => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized"})),
            ),
            DataError::InsertFailed => todo!(),
            DataError::DeleteFailed => todo!(),
            DataError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal Server Error"})),
            ),
        }
    }
}
