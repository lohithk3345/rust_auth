pub type Result<T> = core::result::Result<T, StoreError>;

#[derive(Debug)]
pub enum StoreError {
    ConnectionFailed(&'static str)
}
