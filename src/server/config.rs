use crate::server::logger::configure_bunyan_logger_format;
use dotenvy::dotenv;
use openssl::rand::rand_bytes;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::fmt;
use std::io::Stdout;
use std::net::{IpAddr, Ipv4Addr, TcpListener};
use std::thread::available_parallelism;
use tracing::dispatcher;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use crate::server::models::key_algorithms::openssl::seed_all_algorithms;

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_DB_MAX_CONNECTIONS: u32 = 5;

#[derive(Debug)]
pub enum ServerPort {
    Empty,
    Is(u16),
}
impl Default for ServerPort {
    fn default() -> Self {
        ServerPort::Is(8080)
    }
}

impl fmt::Display for ServerPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerPort::Empty => write!(f, "empty"),
            ServerPort::Is(port) => write!(f, "{}", port),
        }
    }
}
#[derive(Debug)]
pub enum ServerListeningAddress {
    Empty,
    Is(IpAddr),
}
impl Default for ServerListeningAddress {
    fn default() -> Self {
        ServerListeningAddress::Is(IpAddr::V4(Ipv4Addr::LOCALHOST))
    }
}

impl fmt::Display for ServerListeningAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerListeningAddress::Empty => write!(f, "empty"),
            ServerListeningAddress::Is(addr) => write!(f, "{}", addr),
        }
    }
}

pub struct ServerRunTimeConfig {
    pub server_port: ServerPort,
    pub server_address: ServerListeningAddress,
    pub log_level: EnvFilter,
    pub worker_threads: u16,
    pub listener: TcpListener,
    pub db_pool: PgPool, // Add database pool
}

/// Configure database connection pool
async fn configure_database() -> Result<PgPool, Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");

    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(DEFAULT_DB_MAX_CONNECTIONS);

    tracing::info!(
        "Configuring database pool with max {} connections",
        max_connections
    );

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connection pool established");

    Ok(pool)
}

/// Try to bind a listener to the supplied address **first as IPv6**, then IPv4.
fn bind_listener(addr_str: &str, port: u16) -> Result<TcpListener, std::io::Error> {
    if let Ok(ipv6) = addr_str.parse::<std::net::Ipv6Addr>() {
        match TcpListener::bind((ipv6, port)) {
            Ok(l) => return Ok(l),
            Err(e) => tracing::warn!("IPv6 bind failed on [{}]: {} — trying IPv4", addr_str, e),
        }
    }

    if let Ok(ipv4) = addr_str.parse::<std::net::Ipv4Addr>() {
        return TcpListener::bind((ipv4, port));
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrNotAvailable,
        format!("Address '{}' is neither valid IPv4 nor IPv6", addr_str),
    ))
}
/// Generates random bytes and checks for errors.
///
/// This function attempts to fill a buffer with 16 random bytes using the openssl `rand_bytes` function.
/// This is to make sure that the environment is secure for cryptographic applications.
///
/// # Returns
///
/// * `Ok(())` - If random bytes are successfully generated and the buffer is filled.
/// * `Err(Box<dyn std::error::Error>)` - If an error occurs while generating random bytes.
///
/// # Errors
///
/// This function propagates any errors returned by the `rand_bytes` function.
///
/// # Examples
///
/// ```rust
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     check_rng()?;
///     println!("Random bytes successfully generated!");
///     Ok(())
/// }
/// ```
fn check_rng() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Checking OpenSSL random bytes...");
    let mut buf = [0u8; 16];
    rand_bytes(&mut buf)?;
    Ok(())
}


/// Configure Skinnycert environment, optionally using the provided address and port.
/// If parameters are Empty, falls back to `.env` values or defaults.
pub async fn configure_environment(
    server_listening_address: ServerListeningAddress,
    server_port: ServerPort,
    worker_threads_override: Option<u16>,
) -> Result<ServerRunTimeConfig, Box<dyn std::error::Error>> {
    // --- Load environment variables if available ---
    let _ = dotenv();

    // --- Configure logging ---
    let log_level = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer: BunyanFormattingLayer<fn() -> Stdout> = configure_bunyan_logger_format();
    if !dispatcher::has_been_set() {
        let subscriber = Registry::default()
            .with(log_level.clone())
            .with(JsonStorageLayer)
            .with(formatting_layer);

        set_global_default(subscriber).expect("Failed to set tracing subscriber");
    }

    tracing::info!("Logger initialised; starting configuration of environment.");
    check_rng().unwrap_or_else(|e| {
        tracing::error!("RNG check failed: {}", e);
        panic!("OpenSSL failed to generate random bytes, environment is not secured for cryptography applications.");
    });
    tracing::info!("Environment reported secure openssl random bytes.");
    // --- Resolve server port ---
    let mut resolved_port: u16 = match server_port {
        ServerPort::Is(p) => p,
        ServerPort::Empty => std::env::var("SERVER_PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT),
    };

    tracing::info!("Configuring server port: {}", resolved_port);

    // --- Resolve server address ---
    let resolved_address: IpAddr = match server_listening_address {
        ServerListeningAddress::Is(ip) => ip,
        ServerListeningAddress::Empty => std::env::var("SERVER_ADDRESS")
            .ok()
            .and_then(|s| s.parse::<IpAddr>().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
    };
    tracing::info!("Configuring server address: {}", resolved_address);

    let num_cpus = available_parallelism().unwrap().get().to_string();
    tracing::info!("Detected {} CPU cores", num_cpus);

    // --- Worker thread count ---
    let worker_threads: u16 = match worker_threads_override {
        Some(threads) => {
            tracing::info!("Using worker threads from override: {}", threads);
            threads
        }
        None => {
            let threads = std::env::var("WORKER_THREADS")
                .ok()
                .and_then(|s| s.parse::<u16>().ok())
                .unwrap_or(num_cpus.parse::<u16>().unwrap_or(4));
            tracing::info!("Using worker threads: {}", threads);
            threads
        }
    };

    // --- Configure database connection ---
    let db_pool = configure_database().await?;
    seed_all_algorithms(&db_pool)
        .await
        .expect("Failed to configure ECDSA algorithms");
    tracing::info!("ECDSA algorithms configured");
    // --- Bind the listener (IPv6 first, fallback to IPv4) ---
    let listener = bind_listener(&resolved_address.to_string(), resolved_port)
        .map_err(|e| format!("Failed to bind listener: {}", e))?;

    let local_addr = listener.local_addr().expect("Cannot get local address");
    if local_addr.port() != resolved_port {
        resolved_port = local_addr.port();
    }

    tracing::info!(
        "Skinnycert server configured at {}:{} ({} threads).",
        resolved_address,
        resolved_port,
        worker_threads
    );

    Ok(ServerRunTimeConfig {
        server_port: ServerPort::Is(resolved_port),
        server_address: ServerListeningAddress::Is(resolved_address),
        log_level,
        worker_threads,
        listener,
        db_pool,
    })
}
