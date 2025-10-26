use clap::Parser;
use skinnycert::server::configuration::{configure_environment, ServerListeningAddress, ServerPort};
use skinnycert::server::system::run;
use std::net::IpAddr;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Server listening address, e.g. 127.0.0.1
    #[arg(short, long)]
    address: Option<IpAddr>,

    /// Server port, e.g. 8080
    #[arg(short, long)]
    port: Option<u16>,

    /// Number of worker threads
    #[arg(short, long)]
    workers: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    let server_address = match cli.address {
        Some(ip) => ServerListeningAddress::Is(ip),
        None => ServerListeningAddress::Empty,
    };

    let server_port = match cli.port {
        Some(port) => ServerPort::Is(port),
        None => ServerPort::Empty,
    };

    // Configure environment
    let mut config = match configure_environment(server_address, server_port) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };

    // Override worker threads if provided
    if let Some(workers) = cli.workers {
        if workers == 0 {
            eprintln!("Worker threads must be > 0");
            std::process::exit(1);
        }
        config.worker_threads = workers;
        tracing::info!("Worker threads overridden via CLI: {}", workers);
    }

    // Run the server using the pre-bound listener
    run(config.listener, config.worker_threads)?.await?;

    Ok(())
}