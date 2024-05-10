pub type Result<T> = core::result::Result<T, ConfigError>;

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ConfigError {
    ConfigMissingVar(&'static str),
    FailedToLoadEnvFile(&'static str),
}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ConfigError {}

