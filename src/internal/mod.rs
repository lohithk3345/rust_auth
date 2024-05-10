mod auth;
mod data;
mod repository;
mod store;
mod types;

pub mod error;
pub mod models;
pub mod services;
pub mod web;

use std::sync::Mutex;

use mongodb::{ClientSession, Database};
use redis::Connection;

use crate::internal::error::InternalError;

use self::store::{get_mongo_conn_pool_db, get_redis_cache_conn, CacheConn, Db};

#[derive(Clone)]
pub struct InternalManager {
    db: Db,
    cache: CacheConn,
}

impl InternalManager {
    pub async fn new() -> self::error::Result<Self> {
        Ok(Self {
            db: get_mongo_conn_pool_db()
                .await
                .map_err(|err| InternalError::Store(err))?,
            cache: get_redis_cache_conn()
                .await
                .map_err(|err| InternalError::Store(err))?,
        })
    }

    pub(in crate::internal) fn mongo_client(&self) -> Db {
        self.db.clone()
    }

    pub(in crate::internal) fn db(&self, name: &'static str) -> Database {
        self.db.database(name).clone()
    }

    pub(in crate::internal) fn cache(&self, name: &'static str) -> CacheConn {
        self.cache.clone()
    }
}
