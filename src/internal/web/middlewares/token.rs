use axum::{
    body::Body,
    extract::State,
    http::{self, header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{
    core::constants::X_API_KEY,
    internal::{types::TokenAndClaims, InternalManager},
};

use super::super::handlers::token::TokenHandlers;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub(in crate::internal::web) async fn access_token_auth(
        State(manager): State<InternalManager>,
        mut req: Request<Body>,
        next: Next,
    ) -> Result<Response, Response<Body>> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                let result =
                    TokenHandlers::access_token_validation(token.to_string(), manager).await?;
                dbg!(&result);
                req.extensions_mut().insert(result);
                return Ok(next.run(req).await);
            } else {
                return Err(StatusCode::UNAUTHORIZED.into_response());
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        };
    }

    pub(in crate::internal::web) async fn refresh_token_auth(
        State(manager): State<InternalManager>,
        mut req: Request<Body>,
        next: Next,
    ) -> Result<Response, Response<Body>> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                let result =
                    TokenHandlers::refresh_token_validation(&token.to_string(), manager).await?;
                dbg!(&result);
                let data = TokenAndClaims::new(result, token.to_string());
                req.extensions_mut().insert(data);
                return Ok(next.run(req).await);
            } else {
                return Err(StatusCode::UNAUTHORIZED.into_response());
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        };
    }

    pub(in crate::internal::web) async fn api_key_auth(
        req: Request<Body>,
        next: Next,
    ) -> Result<Response<Body>, Response<Body>> {
        let auth_header = req
            .headers()
            .get(X_API_KEY)
            .and_then(|header| header.to_str().ok());

        if let Some(key) = auth_header {
            if TokenHandlers::api_key_check(key.to_owned()) {
                return Ok(next.run(req).await);
            } else {
                return Err(StatusCode::UNAUTHORIZED.into_response());
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED.into_response());
        };
    }
}

// pub(in crate::internal::web) async fn auth(req: Request<Body>, next: Next) -> Result<Response, Response<Body>> {
//     let auth_header = req
//         .headers()
//         .get(http::header::AUTHORIZATION)
//         .and_then(|header| header.to_str().ok());

//     if let Some(auth_header) = auth_header {
//         let result = TokenHandlers::access_token_validation(auth_header.to_string()).await?;
//         return Ok(next.run(req).await);
//     } else {
//         return Err(StatusCode::UNAUTHORIZED.into_response());
//     };
// }

// pub(in crate::internal::web) async fn api_key_auth(
//     req: Request<Body>,
//     next: Next,
// ) -> Result<Response<Body>, Response<Body>> {
//     let auth_header = req
//         .headers()
//         .get("X-API-KEY")
//         .and_then(|header| header.to_str().ok());

//     if let Some(key) = auth_header {
//         if TokenHandlers::api_key_check(key.to_owned()) {
//             return Ok(next.run(req).await);
//         } else {
//             // let res = (StatusCode::UNAUTHORIZED, json(json!({"status": "Invalid Token"}).as_str().unwrap()));
//             // let res = json!({"status_code": StatusCode::UNAUTHORIZED.to_string(), "error": "Invalid Token"});
//             return Err(StatusCode::UNAUTHORIZED.into_response());
//         }
//     } else {
//         return Err(StatusCode::UNAUTHORIZED.into_response());
//     };
// }
