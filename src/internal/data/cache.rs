use super::error::{DataError, Result};
use redis::{AsyncCommands, LposOptions};

use crate::internal::{
    store::CacheConn,
    types::{AccessToken, CacheTokenMap, GetToken, RefreshToken, SetToken, Token, UserID},
};

#[derive(Debug, Clone)]
pub struct Cache {
    cache: CacheConn,
}

impl Cache {
    pub fn new(cache: CacheConn) -> Self {
        Self { cache }
    }

    pub async fn is_access_token_present(
        &mut self,
        id: &UserID,
        token_id: &String,
        token: impl Into<String>,
    ) -> Result<()> {
        let response: Option<String> = self.cache.hget(id, token_id).await.map_err(|err| {
            dbg!(err);
            DataError::NotFound
        })?;

        match response {
            Some(data) => {
                if data.clone().contains(&token.into()) {
                    return Ok(());
                }
                return Err(DataError::NotFound);
            }
            None => {
                return Err(DataError::NotFound);
            }
        }

        Err(DataError::Unknown)
    }

    pub async fn is_refresh_token_present(
        &mut self,
        id: &UserID,
        token_id: &String,
        token: impl Into<String>,
    ) -> Result<()> {
        let response: Option<String> = self.cache.hget(id, token_id).await.map_err(|err| {
            dbg!(err);
            DataError::NotFound
        })?;

        match response {
            Some(data) => {
                if data.clone().contains(&token.into()) {
                    return Ok(());
                }
                return Err(DataError::NotFound);
            }
            None => {
                return Err(DataError::NotFound);
            }
        }

        Err(DataError::Unknown)
    }

    pub async fn insert_access_and_refresh_tokens(
        &mut self,
        id: &UserID,
        token_id: String,
        map: CacheTokenMap,
    ) -> Result<()> {
        let value = serde_json::to_string(&map).map_err(|err| DataError::Unknown)?;

        let inserted = self.cache.hset(id, token_id, value).await.map_err(|err| {
            dbg!(err);
            DataError::InsertFailed
        })?;

        Ok(())
    }

    pub async fn delete_access_and_refresh_token(
        &mut self,
        id: &UserID,
        token_id: String,
    ) -> Result<()> {
        let deleted = self.cache.hdel(id, token_id).await.map_err(|err| {
            dbg!(err);
            DataError::DeleteFailed
        })?;

        Ok(())
    }
}
