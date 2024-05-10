use mongodb::{bson::doc, ClientSession, Collection};

use crate::{
    core::constants::{USER_COLLECTION_NAME, USER_DATABASE_NAME},
    internal::{
        models::user::User, repository::user::UserRepository, store::StoreData, types::InsertID,
        InternalManager,
    },
};

use super::error::{DataError, Result};

pub struct UserRepositoryImpl {
    collection: Collection<User>,
}

impl StoreData for UserRepositoryImpl {
    const DATABASE: &'static str = USER_DATABASE_NAME;

    const COLLECTION: &'static str = USER_COLLECTION_NAME;
}

impl UserRepositoryImpl {
    pub fn new(manager: &InternalManager) -> Self {
        Self {
            collection: manager.db(Self::DATABASE).collection(Self::COLLECTION),
        }
    }
}

impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<InsertID> {
        self.collection
            .insert_one(user, None)
            .await
            .map_err(|err| DataError::handle_insert(err))
            .map(|inserted| inserted.inserted_id.to_string())
    }

    async fn get_by_id(&self, id: impl Into<String>) -> Result<User> {
        self.collection
            .find_one(doc! {"uuid": id.into()}, None)
            .await
            .map_err(|err| {
                dbg!(err);
                DataError::Unknown
            })
            .and_then(|value| value.ok_or(DataError::NotFound))
    }

    async fn get_by_email(&self, email: impl Into<String>) -> Result<User> {
        self.collection
            .find_one(doc! {"email": email.into()}, None)
            .await
            .map_err(|err| {
                dbg!(err);
                DataError::Unknown
            })
            .and_then(|value| value.ok_or(DataError::NotFound))
    }
}
