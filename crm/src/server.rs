use anyhow::Result;
use crm::{AppConfig, CrmService, UserService};
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let addr = format!("[::1]:{}", config.server.port).parse().unwrap();

    let crm_service = CrmService::try_new().await?;
    let user_service = UserService::default();

    info!("CrmServer listening on {}", addr);

    Server::builder()
        .add_service(user_service.into_server())
        .add_service(crm_service.into_server())
        .serve(addr)
        .await?;

    Ok(())
}
