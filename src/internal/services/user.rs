use mongodb::bson::Uuid;

use crate::internal::{
    data::user::UserRepositoryImpl,
    models::user::{GetUserRes, User, UserRegisterReq},
    repository::user::UserRepository,
    types::UserID,
    InternalManager,
};

use super::error::{Result, ServiceError};

pub(in crate::internal) struct UserServices<T: UserRepository> {
    repostiory: T,
}

impl UserServices<UserRepositoryImpl> {
    pub fn new(manager: &InternalManager) -> Self {
        Self {
            repostiory: UserRepositoryImpl::new(manager),
        }
    }

    pub async fn register(&self, user_req: UserRegisterReq) -> Result<GetUserRes> {
        // user.uuid = Uuid::new().to_string();
        let user = User::from_req(user_req)?;
        let user_id = self
            .repostiory
            .create(&user)
            .await
            .map_err(ServiceError::from_data_error)?;
        Ok(user.to_get_user_res())
    }

    pub async fn get_user(&self, id: &UserID) -> Result<GetUserRes> {
        Ok(self
            .repostiory
            .get_by_id(id)
            .await
            .map_err(ServiceError::from_data_error)
            .map(|user| user.to_get_user_res())?)
    }
}
