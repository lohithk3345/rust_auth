use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::auth::Claims;

pub type InsertID = String;
pub type UserID = String;
pub type Token = String;
pub type CacheTokenMap = HashMap<String, Token>;

#[derive(Debug, Clone)]
pub struct TokenAndClaims {
    claims: Claims,
    token: Token,
}

impl TokenAndClaims {
    pub fn new(claims: Claims, token: Token) -> Self {
        Self { claims, token }
    }

    pub fn claims(&self) -> Claims {
        self.claims.clone()
    }

    pub fn token(&self) -> Token {
        self.token.clone()
    }
}

pub trait GetToken {
    fn token(self) -> Token;
}

pub trait SetToken {
    fn set_token(id: String, token: Token) -> Self;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessToken {
    user_id: UserID,
    token: Token,
}

impl GetToken for AccessToken {
    fn token(self) -> Token {
        self.token
    }
}

impl SetToken for AccessToken {
    fn set_token(id: String, token: Token) -> Self {
        Self { user_id: id, token }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshToken {
    user_id: UserID,
    token: Token,
}

impl GetToken for RefreshToken {
    fn token(self) -> Token {
        self.token
    }
}

impl SetToken for RefreshToken {
    fn set_token(id: String, token: Token) -> Self {
        Self { user_id: id, token }
    }
}
