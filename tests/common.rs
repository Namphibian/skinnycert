use skinnycert::server::configuration::{
    ServerListeningAddress, ServerPort, configure_environment,
};
use std::net::{IpAddr, Ipv4Addr};

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
pub fn spawn_app() -> String {
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