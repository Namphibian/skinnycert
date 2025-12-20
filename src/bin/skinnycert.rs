use clap::Parser;
use skinnycert::server::app::run;
use skinnycert::server::config::{ServerListeningAddress, ServerPort, configure_environment};
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

    // Validate worker threads if provided
    if let Some(workers) = cli.workers {
        if workers == 0 {
            eprintln!("Worker threads must be > 0");
            std::process::exit(1);
        }
    }

    // Configure environment with all parameters
    let config = match configure_environment(server_address, server_port, cli.workers).await {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Run the server using the pre-bound listener
    run(config.listener, config.worker_threads, config.db_pool)?.await?;

    Ok(())
}
