use crate::internal::{data::error::Result, models::user::User, store::StoreData, types::InsertID};

pub(in crate::internal) trait UserRepository
where
    Self: StoreData,
{
    async fn create(&self, user: &User) -> Result<InsertID>;
    async fn get_by_id(&self, id: impl Into<String>) -> Result<User>;
    async fn get_by_email(&self, email: impl Into<String>) -> Result<User>;
}
