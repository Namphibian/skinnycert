use skinnycert::server::configuration::{configure_environment, ServerListeningAddress, ServerPort};
use std::net::{IpAddr, Ipv4Addr};
use reqwest;

async fn spawn_app_with_env(env: &str) -> String {
    let env = Some(env.into());

    let config = configure_environment(
        ServerListeningAddress::Is(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        ServerPort::Is(0),
        Some(1),
        env,
    )
    .await
    .expect("Failed to configure environment");

    let server = skinnycert::server::startup::run(
        config.listener,
        config.worker_threads,
        config.db_pool,
        config.environment,
    )
    .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://{}:{}", config.server_address, config.server_port)
}

#[tokio::test]
async fn test_swagger_ui_availability() {
    // Test for 'dev' environment
    let base_url = spawn_app_with_env("dev").await;
    let client = reqwest::Client::new();
    
    let response = client.get(format!("{}/swagger-ui/", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), 200, "Swagger UI should be available in 'dev' environment");

    let response = client.get(format!("{}/api-docs/openapi.json", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_eq!(response.status(), 200, "API docs should be available in 'dev' environment");
}

#[tokio::test]
async fn test_swagger_ui_inaccessibility_in_prod() {
    // Test for 'PROD' environment (uppercase as in config.rs default)
    let base_url = spawn_app_with_env("PROD").await;
    let client = reqwest::Client::new();
    
    let response = client.get(format!("{}/swagger-ui/", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    // In prod, this should NOT be 200. Ideally 404.
    assert_ne!(response.status(), 200, "Swagger UI should NOT be available in 'PROD' environment");

    let response = client.get(format!("{}/api-docs/openapi.json", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_ne!(response.status(), 200, "API docs should NOT be available in 'PROD' environment");
}

#[tokio::test]
async fn test_swagger_ui_inaccessibility_in_prod_lowercase() {
    // Test for 'prod' environment (lowercase)
    let base_url = spawn_app_with_env("prod").await;
    let client = reqwest::Client::new();
    
    let response = client.get(format!("{}/swagger-ui/", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    // In prod, this should NOT be 200. Ideally 404.
    assert_ne!(response.status(), 200, "Swagger UI should NOT be available in 'prod' environment");

    let response = client.get(format!("{}/api-docs/openapi.json", base_url))
        .send()
        .await
        .expect("Failed to send request");
    
    assert_ne!(response.status(), 200, "API docs should NOT be available in 'prod' environment");
}
