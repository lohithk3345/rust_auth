use bcrypt::{hash, DEFAULT_COST};
use mongodb::bson::Uuid;
use serde::{Deserialize, Serialize};

use crate::internal::services::error::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub hash: String,
    pub age: i32,
    pub created_at: usize,
}

impl User {
    pub fn from_req(user_req: UserRegisterReq) -> Result<Self, ServiceError> {
        let id = Uuid::new();
        Ok(Self {
            email: user_req.email,
            name: user_req.name,
            age: user_req.age,
            uuid: id.to_string(),
            hash: hash(user_req.password, DEFAULT_COST).map_err(|err| {
                dbg!(err);
                ServiceError::Unknown
            })?,
            created_at: chrono::Utc::now().timestamp_millis() as usize,
        })
    }

    pub fn to_get_user_res(self) -> GetUserRes {
        GetUserRes {
            name: self.name,
            age: self.age,
            email: self.email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisterReq {
    pub email: String,
    pub password: String,
    pub age: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserRes {
    pub email: String,
    pub age: i32,
    pub name: String,
}
