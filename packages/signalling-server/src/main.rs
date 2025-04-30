mod bootstrap;

use tokio::signal;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber;
use tracing_subscriber::EnvFilter;

/// Main bootstrapping function for the signalling server.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file, if present
    match dotenvy::dotenv() {
        Ok(_) => println!("Loaded environment variables from .env file"),
        Err(e) => println!("No .env file found: {}", e),
    }

    let configuration = bootstrap::get_configuration_from_env()
        .expect("failed to get config from environment.");

    // Set the logger to debug level by default, can be overridden by the RUST_LOG envvar
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env()?;

    tracing_subscriber::fmt::init();

    // TODO: Spawn the signalling server task

    match signal::ctrl_c().await {
        Ok(()) => {},
        Err(err) => {
            tracing::error!("Failed to listen for shutdown signal: {}", err);
            // we also shut down in case of error
        },
    }

    // TODO: Gracefully shut down the signalling server task

    Ok(())
}
