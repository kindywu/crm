use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use crm_send::{
    notification_client::NotificationClient, notification_server::NotificationServer, AppConfig,
    EmailMessage, InAppMessage, NotificationService, SmsMessage,
};
use tokio::{spawn, time::sleep};
use tokio_stream::StreamExt;
use tonic::{transport::Server, Request};
use tracing::info;

#[tokio::test]
async fn test_send_should_work() -> Result<()> {
    let addr = start_server().await?;

    sleep(Duration::from_secs(1)).await;

    start_client(addr).await?;
    Ok(())
}

async fn start_client(addr: SocketAddr) -> Result<()> {
    let addr = format!("http://{addr}");
    // println!("{addr:?}");
    let mut client = NotificationClient::connect(addr).await?;
    let stream = tokio_stream::iter(vec![
        EmailMessage::fake().into(),
        SmsMessage::fake().into(),
        InAppMessage::fake().into(),
    ]);
    let request = Request::new(stream);

    let response = client.send(request).await?;

    let ret: Vec<_> = response
        .into_inner()
        .then(|r| async { r.unwrap() })
        .collect()
        .await;

    assert_eq!(ret.len(), 3);
    Ok(())
}

async fn start_server() -> Result<SocketAddr> {
    let config = AppConfig::load()?;

    let addr = format!("[::1]:{}", config.server.port).parse()?;
    info!("NotificationServer listening on {}", addr);

    let svc = NotificationService::new(config);

    spawn(async move {
        Server::builder()
            .add_service(NotificationServer::new(svc))
            .serve(addr)
            .await
            .unwrap();
    });

    Ok(addr)
}
