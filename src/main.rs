#![allow(unused)]
mod internal;

use std::time::Instant;

use axum::{extract::State, routing::get, Json, Router};
use serde_json::json;
use tokio::net::TcpListener;

use crate::{
    core::constants::API_ROUTE_PREFACE,
    internal::{web::apis::user::user_routes, InternalManager},
};
mod config;
mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let manager = InternalManager::new().await?;
    let addr = format!("0.0.0.0:3000");
    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        Router::new()
            .nest_service(
                API_ROUTE_PREFACE,
                user_routes(State(manager.clone())).route(
                    "/health",
                    get(|| async {
                        let start = Instant::now();
                        dbg!("CHECK");
                        let end = Instant::now();
                        dbg!(end - start);
                        Json(json!({"STATUS":"UP"}))
                    }),
                ),
            )
            .into_make_service(),
    )
    .await?;
    Ok(())
}
