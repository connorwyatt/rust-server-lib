use std::net::{SocketAddr, TcpListener};

use axum::{debug_handler, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;

pub(crate) fn create() -> SocketAddr {
    server_testing::create_test_server(app())
}

fn app() -> Router {
    server::create_router(Router::new().route("/test", get(handler)))
}

async fn handler() -> StatusCode {
    StatusCode::OK
}