mod test_server;

use std::str::FromStr;

use axum::http::{Response, StatusCode};
use hyper::{client::Client, Body, Uri};

#[tokio::test]
async fn should_respond_to_requests() {
    let socket_addr = test_server::create();

    let client = Client::new();

    let uri = Uri::from_str(format!("http://localhost:{}/test", socket_addr.port()).as_str())
        .expect("uri should be parseable");

    let response = client.get(uri).await.expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_generate_and_add_x_request_id_to_the_response_when_one_is_not_provided() {
    let socket_addr = test_server::create();

    let client = Client::new();

    let uri =
        Uri::from_str(format!("http://localhost:{}/test", socket_addr.port()).as_str()).expect("");

    let response = assert_and_unwrap_response(client.get(uri).await);

    let x_request_id = get_x_request_id(response);

    assert!(!x_request_id.is_empty());
}

fn assert_and_unwrap_response(result: hyper::Result<Response<Body>>) -> Response<Body> {
    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    response
}

fn get_x_request_id(response: Response<Body>) -> String {
    let x_request_id_header = response
        .headers()
        .get("X-Request-Id")
        .expect("X-Request-Id should be set on the response");

    assert!(!x_request_id_header.is_empty());

    let x_request_id = x_request_id_header
        .to_str()
        .expect("X-Request-Id should be valid");

    x_request_id.to_string()
}
