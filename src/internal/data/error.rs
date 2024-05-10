use mongodb::error::WriteFailure;

pub type Result<T> = core::result::Result<T, DataError>;

// #[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum DataError {
    InsertFailed,
    DeleteFailed,
    Duplicate,
    NotFound,
    Unknown,
}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DataError {}

impl DataError {
    pub fn handle_insert(err: mongodb::error::Error) -> Self {
        match *err.kind {
            mongodb::error::ErrorKind::Write(WriteFailure::WriteError(e)) => {
                if e.code == 11000 {
                    return DataError::Duplicate;
                }
                return DataError::Unknown;
            }
            _ => DataError::Unknown,
        }
    }
}
