pub type Result<T> = core::result::Result<T, InternalError>;

#[derive(Debug)]
pub enum InternalError {
    Store(super::store::error::StoreError)
}

impl core::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for InternalError {}

