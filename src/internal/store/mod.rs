pub mod error;
use mongodb::{options::ClientOptions, Client as MongoClient};
use redis::{aio::MultiplexedConnection, Client as RedisClient, RedisConnectionInfo};

use self::error::{Result, StoreError};
use crate::config::config;

pub type Db = MongoClient;
pub type CacheConn = MultiplexedConnection;

pub(in crate::internal) trait StoreData {
    const DATABASE: &'static str;
    const COLLECTION: &'static str;
}

async fn mongo_config_options() -> Result<ClientOptions> {
    ClientOptions::parse(&config().MONGO_URI)
        .await
        .map_err(|_| StoreError::ConnectionFailed("Failed at database options"))
        .map(|mut options| {
            options.max_pool_size = Some(5);
            options
        })
}

pub(in crate::internal) async fn get_mongo_conn_pool_db() -> Result<Db> {
    let options = mongo_config_options().await?;
    Db::with_options(options)
        .map_err(|_| StoreError::ConnectionFailed("Failed at creating database client"))
}

pub(in crate::internal) async fn get_redis_cache_conn() -> Result<CacheConn> {
    // CacheConn::new(RedisConnectionInfo::o, stream)
    let conn = RedisClient::open("redis://localhost:6379")
        .map_err(|err| {
            dbg!(err);
            StoreError::ConnectionFailed("Opening Cache Conn Failed")
        })?
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| StoreError::ConnectionFailed("Cache Connection Failed"))?;
    Ok(conn)
}
