use skinnycert::server::configuration::{
    ServerListeningAddress, ServerPort, configure_environment,
};
use std::net::{IpAddr, Ipv4Addr};

/// Integration-style unit test for the `/health` endpoint.
///
/// This test spins up a real instance of the Skinnycert server on a random
/// available port, sends an HTTP GET request to `/health`, and verifies
/// that the endpoint:
/// - Responds successfully (2xx status code)
/// - Returns a non-empty response body
///
/// # How it Works
/// - `spawn_app()` configures and launches the server asynchronously using
///   a randomly assigned port (`ServerPort::Is(0)`), ensuring tests
///   don't conflict with other running instances.
/// - The test client uses `reqwest` to perform an HTTP request against the
///   spawned server instance.
/// - Assertions verify the basic health of the endpoint.
///
/// # Running the Test
/// You can run this test with:
/// ```bash
/// cargo test --test health_check
/// ```
///
/// The Tokio runtime is used for async I/O via `#[tokio::test]`.
#[tokio::test]
async fn get_health_check_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app();
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a GET request to the `/health` endpoint.
    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // --- Assert ---
    // The endpoint should return a successful 2xx response.
    assert!(
        response.status().is_success(),
        "Health check did not return a successful status code"
    );

    // The body should not be empty — it should contain JSON data.
    assert!(
        response.content_length().unwrap_or(0) > 0,
        "Health check returned an empty response body"
    );
}

#[tokio::test]
async fn post_health_check_test() {
    // --- Arrange ---
    // Start an ephemeral instance of the server and retrieve its base URL.
    let address = spawn_app();
    println!("Test server listening on {}", address);

    // Initialize an asynchronous HTTP client.
    let client = reqwest::Client::new();

    // --- Act ---
    // Send a GET request to the `/health` endpoint.
    let response = client
        .post(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // --- Assert ---
    // The endpoint should return a successful 2xx response.
    assert!(
        response.status().is_client_error(),
        "We should see a 405 for POST"
    );
}

/// Spawns a new instance of the Skinnycert application bound to an ephemeral port.
///
/// This helper function is used by integration tests to start the server
/// without hardcoding a specific port (using port `0` allows the OS to
/// assign a free one automatically).
///
/// # Returns
/// A `String` representing the base URL of the running server,
/// e.g. `"http://127.0.0.1:34567"`.
///
/// # Panics
/// - If environment configuration fails.
/// - If the server cannot bind to the provided address.
///
/// The server is spawned on a background Tokio task so the test can
/// continue executing requests against it.
fn spawn_app() -> String {
    // Configure the server with localhost and a dynamic (ephemeral) port.
    let config = match configure_environment(
        ServerListeningAddress::Is(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        ServerPort::Is(0), // 0 tells the OS to assign a random free port
    ) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            panic!("Cannot continue test due to configuration error");
        }
    };

    // Launch the Actix system server asynchronously.
    let server = skinnycert::server::system::run(config.listener, config.worker_threads)
        .expect("Failed to bind address");

    // Spawn the server in the background (so test execution can continue).
    let _ = tokio::spawn(server);

    // Construct and return the full base URL for HTTP requests.
    format!("http://{}:{}", config.server_address, config.server_port)
}
