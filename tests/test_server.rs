#![allow(dead_code)]

use std::net::SocketAddr;

use axum::{http::StatusCode, routing::get, Router};

pub(crate) fn create() -> SocketAddr {
    server_testing::create_test_server(app())
}

fn app() -> Router {
    server::create_router(Router::new().route("/test", get(handler)))
}

async fn handler() -> StatusCode {
    StatusCode::OK
}
