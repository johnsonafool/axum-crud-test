mod handlers;
mod book;
use crate::book::Book;
mod db;
use crate::db::DATA;

use axum::routing::get;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::collections::HashMap;
/// Use Thread for spawning a thread e.g. to acquire our DATA mutex lock.
use std::thread;

#[tokio::main]
pub async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
    // .fallback(
    //     fallback
    // )
    .route("/",
        get(handlers::health)
    );        

    // with socket address
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}