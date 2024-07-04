use anyhow::Result;
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
use user_stat::{user_stat_server::UserStatServer, AppConfig, AppState, UserStatService};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse()?;

    info!("UserStatServer listening on {}", addr);
    let app_state = AppState::try_new(config).await?;
    let svc = UserStatService::new(app_state).await?;
    Server::builder()
        .add_service(UserStatServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
