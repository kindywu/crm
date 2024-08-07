use anyhow::Result;
use crm_send::{notification_server::NotificationServer, AppConfig, NotificationService};
use tonic::transport::Server;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = AppConfig::load()?;
    let addr = config.server.port;
    let addr = format!("[::1]:{}", addr).parse()?;

    info!("NotificationServer listening on {}", addr);

    let svc = NotificationService::new(config);
    Server::builder()
        .add_service(NotificationServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}
