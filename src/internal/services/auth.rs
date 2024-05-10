use bcrypt::{verify, BcryptError};

use uuid::Uuid;

use crate::{
    config::config,
    core::constants::{ACCESS_TOKEN_CACHE_KEY, REFRESH_TOKEN_CACHE_KEY},
    internal::{
        auth::TokenRepository,
        data::{cache::Cache, user::UserRepositoryImpl},
        models::{token::Tokens, user::UserLoginReq},
        repository::user::UserRepository,
        store::CacheConn,
        types::{CacheTokenMap, Token, UserID},
        InternalManager,
    },
};

use super::error::{Result, ServiceError};

pub(in crate::internal) struct AuthServices<T: UserRepository> {
    repostiory: T,
    token_service: TokenRepository,
    cache: Cache,
}

impl AuthServices<UserRepositoryImpl> {
    pub fn new(manager: &InternalManager) -> Self {
        Self {
            repostiory: UserRepositoryImpl::new(&manager.clone()),
            token_service: TokenRepository::new(),
            cache: Cache::new(manager.clone().cache),
        }
    }

    pub async fn login(&mut self, user_req: &UserLoginReq) -> Result<Tokens> {
        let user = self
            .repostiory
            .get_by_email(&user_req.username)
            .await
            .map_err(ServiceError::from_data_error)?;

        let is_authorized =
            bcrypt::verify(user_req.clone().password, &user.hash).map_err(|err| {
                dbg!(err);
                ServiceError::UserPasswordUnauthorized
            })?;

        if !is_authorized {
            return Err(ServiceError::UserPasswordUnauthorized);
        }

        let token_id = Uuid::new_v4().to_string();

        let access = self
            .token_service
            .create_access_token(&user.uuid, token_id.clone(), &config().ACCESS_TOKEN_SECRET)
            .map_err(ServiceError::from_token_error)?;

        let refresh = self
            .token_service
            .create_refresh_token(&user.uuid, &token_id, &config().REFRESH_TOKEN_SECRET)
            .map_err(ServiceError::from_token_error)?;

        let mut map = CacheTokenMap::new();
        map.insert(ACCESS_TOKEN_CACHE_KEY.to_string(), access.clone());
        map.insert(REFRESH_TOKEN_CACHE_KEY.to_string(), refresh.clone());

        self.cache
            .insert_access_and_refresh_tokens(&user.uuid, token_id, map)
            .await
            .map_err(ServiceError::from_data_error)?;

        Ok(Tokens {
            access_token: access,
            refresh_token: refresh,
        })
    }

    pub async fn logout(&mut self, id: UserID, token_id: String) -> Result<()> {
        self.cache
            .delete_access_and_refresh_token(&id, token_id)
            .await
            .map_err(|err| {
                dbg!(&err);
                ServiceError::from_data_error(err)
            })?;

        Ok(())
    }

    pub async fn refresh_token(
        &mut self,
        id: UserID,
        token_id: String,
        token: Token,
    ) -> Result<Tokens> {
        self.cache
            .is_refresh_token_present(&id, &token_id, token)
            .await
            .map_err(|err| {
                dbg!(&err);
                ServiceError::from_data_error(err)
            })?;

        self.cache
            .delete_access_and_refresh_token(&id, token_id)
            .await
            .map_err(|err| {
                dbg!(&err);
                ServiceError::from_data_error(err)
            })?;

        let token_id = Uuid::new_v4().to_string();

        let access = self
            .token_service
            .create_access_token(&id, token_id.clone(), &config().ACCESS_TOKEN_SECRET)
            .map_err(ServiceError::from_token_error)?;

        let refresh = self
            .token_service
            .create_refresh_token(&id, &token_id, &config().REFRESH_TOKEN_SECRET)
            .map_err(ServiceError::from_token_error)?;

        let mut map = CacheTokenMap::new();
        map.insert(ACCESS_TOKEN_CACHE_KEY.to_string(), access.clone());
        map.insert(REFRESH_TOKEN_CACHE_KEY.to_string(), refresh.clone());

        self.cache
            .insert_access_and_refresh_tokens(&id, token_id, map)
            .await
            .map_err(ServiceError::from_data_error)?;

        Ok(Tokens {
            access_token: access,
            refresh_token: refresh,
        })
    }
}
