use super::super::error::Result;
use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    config::config,
    internal::{
        auth::{Claims, TokenRepository},
        data::cache::Cache,
        store::CacheConn,
        types::Token,
        web::error::ApiError,
        InternalManager,
    },
};

pub struct TokenHandlers;

impl TokenHandlers {
    pub fn api_key_check(key: String) -> bool {
        return key == config().API_KEY;
    }

    pub async fn access_token_validation(token: Token, manager: InternalManager) -> Result<Claims> {
        dbg!("Token Verification");
        let claims = TokenRepository::new()
            .validate_access_token(&token.clone(), &config().ACCESS_TOKEN_SECRET)
            .map_err(|err| {
                dbg!("ERROR TOKEN");
                dbg!(err);
                return StatusCode::UNAUTHORIZED.into_response();
            })?;
        // let cache = cache().await;
        let result = Cache::new(manager.cache)
            .is_access_token_present(&claims.id(), &claims.token_id(), &token.clone())
            .await
            .map_err(|err| {
                dbg!(&err);
                ApiError::handle_data_error(&err).into_response()
            })?;
        Ok(claims)
    }

    pub async fn refresh_token_validation(
        token: &String,
        manager: InternalManager,
    ) -> Result<Claims> {
        let claims = TokenRepository::new()
            .validate_refresh_token(token, &config().REFRESH_TOKEN_SECRET)
            .unwrap();
        Cache::new(manager.cache)
            .is_refresh_token_present(&claims.id(), &claims.token_id(), token)
            .await
            .map_err(
                |err| ApiError::Data(err).into_response(), // ApiError::<DataError>::from(err).into_response()
            )?;
        Ok(claims)
    }
}
