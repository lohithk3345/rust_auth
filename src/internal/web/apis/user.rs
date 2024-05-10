use axum::{
    extract::State,
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    core::constants::{
        AUTH_API_ROUTE, GET_USER_API_ROUTE, LOGIN_USER_API_ROUTE, LOGOUT_USER_API_ROUTE,
        REGISTER_ROUTE, USER_API_ROUTE,
    },
    internal::{
        web::{handlers::users::UserHandlers, middlewares::token::AuthMiddleware},
        InternalManager,
    },
};

pub fn user_routes(State(manager): State<InternalManager>) -> Router {
    let access_token_auth =
        middleware::from_fn_with_state(manager.clone(), AuthMiddleware::access_token_auth);
    let refresh_token_auth =
        middleware::from_fn_with_state(manager.clone(), AuthMiddleware::refresh_token_auth);

    let auth_routes = Router::new()
        .route(REGISTER_ROUTE, post(UserHandlers::create_user))
        .route(
            LOGIN_USER_API_ROUTE,
            post(UserHandlers::login).with_state(manager.clone()),
        )
        .route(
            LOGOUT_USER_API_ROUTE,
            get(UserHandlers::logout).layer(access_token_auth.clone()),
        )
        .route(
            GET_USER_API_ROUTE,
            get(UserHandlers::get_user).layer(access_token_auth.clone()),
        )
        .route(
            "/new",
            get(UserHandlers::refresh_tokens).layer(refresh_token_auth.clone()),
        );

    Router::new()
        .nest(
            USER_API_ROUTE,
            Router::new().nest(AUTH_API_ROUTE, auth_routes),
        )
        .layer(axum::middleware::from_fn(AuthMiddleware::api_key_auth))
        .with_state(manager.clone())
}
