use std::time::Instant;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};
use axum_auth::AuthBasic;
use serde_json::json;

use crate::internal::{
    auth::Claims,
    data::user::UserRepositoryImpl,
    models::user::{User, UserLoginReq, UserRegisterReq},
    services::{auth::AuthServices, user::UserServices},
    types::TokenAndClaims,
    InternalManager,
};

pub(in crate::internal::web) struct UserHandlers;

impl UserHandlers {
    pub async fn create_user(
        State(manager): State<InternalManager>,
        Json(user): Json<UserRegisterReq>,
    ) -> impl IntoResponse {
        let start = Instant::now();
        let result = UserServices::<UserRepositoryImpl>::new(&manager)
            .register(user)
            .await;
        let end = Instant::now();
        println!("{:?}", end - start);
        match result {
            Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
            Err(err) => {
                let res = err.get_user_error_response();
                res.into_response()
            }
        }
    }

    pub async fn get_user(
        State(manager): State<InternalManager>,
        Extension(claims): Extension<Claims>,
    ) -> impl IntoResponse {
        let start = Instant::now();
        let result = UserServices::<UserRepositoryImpl>::new(&manager)
            .get_user(claims.id())
            .await;
        let end = Instant::now();
        println!("{:?}", end - start);
        match result {
            Ok(user) => {
                dbg!(&user);
                (StatusCode::FOUND, Json(user)).into_response()
            }
            Err(err) => {
                dbg!(&err);
                let res = err.get_user_error_response();
                res.into_response()
            }
        }
    }

    pub(in crate::internal::web) async fn login(
        State(manager): State<InternalManager>,
        // Json(user): Json<UserLoginReq>,
        AuthBasic((username, password)): AuthBasic,
    ) -> impl IntoResponse {
        let start = Instant::now();
        if let Some(password) = password {
            let user = UserLoginReq {
                username: username,
                password: password,
            };

            let result = AuthServices::<UserRepositoryImpl>::new(&manager)
                .login(&user)
                .await;
            let end = Instant::now();
            println!("{:?}", end - start);
            match result {
                Ok(tokens) => (StatusCode::OK, Json(tokens)).into_response(),
                Err(err) => err.get_user_error_response().into_response(),
            }
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized. Username Or Password Is Incorrect"})),
            )
                .into_response();
        }
    }

    pub(in crate::internal::web) async fn logout(
        State(manager): State<InternalManager>,
        Extension(claims): Extension<Claims>,
    ) -> impl IntoResponse {
        let start = Instant::now();
        let result = AuthServices::<UserRepositoryImpl>::new(&manager)
            .logout(claims.id().to_string(), claims.token_id())
            .await;
        let end = Instant::now();
        println!("{:?}", end - start);
        match result {
            Ok(_) => (StatusCode::OK, Json(json!({"status": "Logged Out"}))).into_response(),
            Err(err) => err.get_user_error_response().into_response(),
        }
    }

    pub(in crate::internal::web) async fn refresh_tokens(
        State(manager): State<InternalManager>,
        Extension(data): Extension<TokenAndClaims>,
    ) -> impl IntoResponse {
        let start = Instant::now();

        let result = AuthServices::<UserRepositoryImpl>::new(&manager)
            .refresh_token(
                data.claims().id().to_string(),
                data.claims().token_id(),
                data.token(),
            )
            .await;

        let end = Instant::now();
        println!("{:?}", end - start);
        match result {
            Ok(tokens) => (StatusCode::OK, Json(json!(tokens))).into_response(),
            Err(err) => err.get_user_error_response().into_response(),
        }
    }
}

// pub(in crate::internal::web) async fn create_user(
//     State(manager): State<InternalManager>,
//     Json(mut user): Json<User>,
// ) -> impl IntoResponse {
//     let start = Instant::now();
//     let result = UserServices::<UserRepositoryImpl>::new(&manager)
//         .register(&mut user)
//         .await;
//     let end = Instant::now();
//     println!("{:?}", end - start);
//     match result {
//         Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
//         Err(err) => {
//             let res = err.get_user_error_response();
//             res.into_response()
//         }
//     }
// }

// pub(in crate::internal::web) async fn login(
//     State(manager): State<InternalManager>,
//     Json(user): Json<UserLoginReq>,
// ) -> impl IntoResponse {
//     let start = Instant::now();
//     let result = AuthServices::<UserRepositoryImpl>::new(&manager)
//         .login(&user)
//         .await;
//     let end = Instant::now();
//     println!("{:?}", end - start);
//     match result {
//         Ok(tokens) => (StatusCode::OK, Json(tokens)).into_response(),
//         Err(err) => err.get_user_error_response().into_response(),
//     }
// }

// pub(in crate::internal::web) async fn refresh_tokens(token: String) {
//     // let result = TokenService::validate_refresh_token(&token);
// }
