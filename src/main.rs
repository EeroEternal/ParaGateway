use paragateway::config::Config;
use paragateway::server;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "paragateway=debug,tower_http=debug,axum::rejection=trace".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load()?;
    
    tracing::info!("Starting ParaGateway on {}", config.addr);
    
    server::run(config).await?;

    Ok(())
}
