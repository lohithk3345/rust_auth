pub mod error;

use self::error::{ConfigError, Result};
use std::{sync::OnceLock, env};
use dotenv::dotenv;



#[allow(non_snake_case)]
#[derive(Clone)]
pub struct Config {
    pub MONGO_URI: String,
    pub PORT: String,
    pub ACCESS_TOKEN_SECRET: String,
    pub REFRESH_TOKEN_SECRET: String,
    pub API_KEY: String,
}

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_env().unwrap_or_else(|err| {
            panic!("ERROR In Initializing Config * REASON: => \" {:?} \"", err)
        })
    })
}

impl Config {
    pub fn load_env() -> Result<Self> {
        if dotenv().is_err() {
            return Err(ConfigError::FailedToLoadEnvFile("Error Loading Env From File"));
        }
        Ok(Self {
            MONGO_URI: get_var("MONGO_DB_URI")?,
            PORT: get_var("SERVE_PORT")?,
            ACCESS_TOKEN_SECRET: get_var("ACCESS_TOKEN_SECRET")?,
            REFRESH_TOKEN_SECRET: get_var("REFRESH_TOKEN_SECRET")?,
            API_KEY: get_var("API_KEY")?,
        })
    }
}

fn get_var(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| {
        ConfigError::ConfigMissingVar(name)
    })
}
