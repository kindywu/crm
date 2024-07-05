use std::{net::SocketAddr, time::Duration};

use anyhow::Result;
use sqlx_db_tester::TestPg;
use tokio::time::sleep;
use tokio_stream::StreamExt;
use tonic::transport::Server;
use user_stat::{
    test_utils::id, user_stat_client::UserStatClient, user_stat_server::UserStatServer, AppState,
    QueryRequestBuilder, RawQueryRequestBuilder, UserStatService,
};

const PORT_BASE: u32 = 60000;

#[tokio::test]
async fn raw_query_should_work() -> Result<()> {
    let (_tdb, addr) = start_server(PORT_BASE).await?;

    let mut client = UserStatClient::connect(format!("http://{addr}")).await?;
    let req = RawQueryRequestBuilder::default()
        .query("SELECT * FROM user_stats WHERE created_at > '2024-01-01' LIMIT 5")
        .build()?;

    let stream = client.raw_query(req).await?.into_inner();
    let ret = stream
        .then(|res| async move { res.unwrap() })
        .collect::<Vec<_>>()
        .await;

    assert_eq!(ret.len(), 5);
    Ok(())
}

#[tokio::test]
async fn query_should_work() -> Result<()> {
    let (_tdb, addr) = start_server(PORT_BASE + 1).await?;
    let mut client = UserStatClient::connect(format!("http://{addr}")).await?;
    let query = QueryRequestBuilder::default()
        // .timestamp(("last_visited_at".to_string(), tq(Some(300), None)))
        .id(("viewed_but_not_started".to_string(), id(&[252790])))
        .build()
        .unwrap();

    let stream = client.query(query).await?.into_inner();
    let ret = stream.collect::<Vec<_>>().await;

    assert_eq!(ret.len(), 16);
    Ok(())
}

async fn start_server(port: u32) -> Result<(TestPg, SocketAddr)> {
    let addr = format!("[::1]:{}", port).parse()?;

    let (tdb, app_state) = AppState::try_new_test().await?;
    let svc = UserStatService::new(app_state).await?;

    tokio::spawn(async move {
        Server::builder()
            .add_service(UserStatServer::new(svc))
            .serve(addr)
            .await
            .unwrap();
    });
    sleep(Duration::from_micros(1)).await;

    Ok((tdb, addr))
}
