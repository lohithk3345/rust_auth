pub mod error;

use std::time::{self, SystemTime, UNIX_EPOCH};

use axum::{body::Body, extract::Request};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{
    core::constants::{HOURS_IN_DAY, ONE_HOUR, REFRESH_TOKEN_EXP_DAYS},
    internal::auth::error::{Result, TokenError},
};

use super::types::{Token, UserID};

pub trait TokenMiddleware {
    fn get_auth_token(req: &Request<Body>) -> Result<String> {
        if req.headers().get("Authorization").is_none() {
            return Err(TokenError::NotFound);
        }
        req.headers()
            .get("Authorization")
            .map(|tok| tok.to_str())
            .unwrap()
            .map_err(|err| {
                dbg!(err);
                TokenError::Unknown
            })
            .map(|tok| tok.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    id: String,
    token_id: String,
    exp: usize,
}

impl Claims {
    pub fn id(&self) -> &UserID {
        &self.id
    }

    pub fn token_id(&self) -> String {
        self.token_id.clone()
    }
}

#[derive(Debug, Clone)]
pub struct TokenRepository;

impl TokenRepository {
    pub fn new() -> Self {
        Self
    }

    pub fn create_access_token(
        &self,
        id: impl Into<String>,
        token_id: impl Into<String>,
        secret: &String,
    ) -> Result<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| TokenError::Unknown)?;
        let exp_time = ONE_HOUR;
        let claims = Claims {
            id: id.into(),
            token_id: token_id.into(),
            exp: (now + time::Duration::from_secs(exp_time)).as_secs() as usize,
        };
        encode(
            &Header::new(jsonwebtoken::Algorithm::HS384),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|err| {
            dbg!(err);
            TokenError::Unknown
        })
    }

    pub fn create_refresh_token(
        &self,
        id: impl Into<String>,
        token_id: impl Into<String>,
        secret: &String,
    ) -> Result<String> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let exp_time = ONE_HOUR * HOURS_IN_DAY * REFRESH_TOKEN_EXP_DAYS;
        let claims = Claims {
            id: id.into(),
            token_id: token_id.into(),
            exp: (now + time::Duration::from_secs(exp_time)).as_secs() as usize,
        };
        encode(
            &Header::new(jsonwebtoken::Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|err| {
            dbg!(err);
            TokenError::Unknown
        })
    }

    pub fn validate_access_token(&self, token: &String, secret: &String) -> Result<Claims> {
        let secret = DecodingKey::from_secret(secret.as_bytes());
        Ok(decode::<Claims>(
            &token,
            &secret,
            &Validation::new(jsonwebtoken::Algorithm::HS384),
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => TokenError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => TokenError::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => TokenError::InvalidSignature,
            _ => TokenError::Unknown,
        })?
        .claims)
    }

    pub fn validate_refresh_token(&self, token: &String, secret: &String) -> Result<Claims> {
        let secret = DecodingKey::from_secret(secret.as_bytes());
        Ok(decode::<Claims>(
            &token,
            &secret,
            &Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken => TokenError::InvalidToken,
            jsonwebtoken::errors::ErrorKind::InvalidSignature => TokenError::InvalidSignature,
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => TokenError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidAlgorithm => TokenError::InvalidSignature,
            _ => TokenError::Unknown,
        })?
        .claims)
    }
}
