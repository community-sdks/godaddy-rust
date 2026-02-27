mod support;

use godaddy_rust::http::Response;
use godaddy_rust::{ApiError, Client, Config};

use support::MockTransport;

fn test_client(transport: std::sync::Arc<MockTransport>) -> Client {
    let mut config = Config::default();
    config.api_key = Some(String::from("key"));
    config.api_secret = Some(String::from("secret"));
    config.max_retries = 0;
    Client::with_transport(config, transport)
}

#[tokio::test]
async fn maps_400_to_validation() {
    let transport = MockTransport::shared();
    transport.push_response(Response::json(400, "{}"));
    let result = test_client(transport)
        .abuse()
        .get_tickets(None, None, None, None, None, None, None, None)
        .await;
    assert!(matches!(result, Err(ApiError::Validation(_))));
}

#[tokio::test]
async fn maps_401_to_unauthorized() {
    let transport = MockTransport::shared();
    transport.push_response(Response::json(401, "{}"));
    let result = test_client(transport)
        .abuse()
        .get_tickets(None, None, None, None, None, None, None, None)
        .await;
    assert!(matches!(result, Err(ApiError::Unauthorized(_))));
}

#[tokio::test]
async fn maps_404_to_not_found() {
    let transport = MockTransport::shared();
    transport.push_response(Response::json(404, "{}"));
    let result = test_client(transport)
        .abuse()
        .get_tickets(None, None, None, None, None, None, None, None)
        .await;
    assert!(matches!(result, Err(ApiError::NotFound(_))));
}

#[tokio::test]
async fn maps_429_to_rate_limit() {
    let transport = MockTransport::shared();
    transport.push_response(Response::json(429, "{}"));
    let result = test_client(transport)
        .abuse()
        .get_tickets(None, None, None, None, None, None, None, None)
        .await;
    assert!(matches!(result, Err(ApiError::RateLimit(_))));
}

#[tokio::test]
async fn maps_500_to_server() {
    let transport = MockTransport::shared();
    transport.push_response(Response::json(500, "{}"));
    let result = test_client(transport)
        .abuse()
        .get_tickets(None, None, None, None, None, None, None, None)
        .await;
    assert!(matches!(result, Err(ApiError::Server(_))));
}
