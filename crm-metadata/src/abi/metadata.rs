use anyhow::Result;
use sqlx::PgPool;
use std::{pin::Pin, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{Response, Status, Streaming};

use crate::{
    config::AppConfig,
    pb::{Content, MaterializeRequest},
};

#[allow(unused)]
pub struct MetadataServiceState {
    config: AppConfig,
    pool: PgPool,
}

#[allow(unused)]
pub struct MetadataService {
    state: Arc<MetadataServiceState>,
}

impl MetadataService {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self {
            state: Arc::new(MetadataServiceState { config, pool }),
        })
    }
}

pub type ServiceResult<T> = Result<Response<T>, Status>;
pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<Content, Status>> + Send>>;

impl MetadataService {
    pub async fn materialize(
        &self,
        mut stream: Streaming<MaterializeRequest>,
    ) -> ServiceResult<ResponseStream> {
        let (tx, rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            while let Some(Ok(req)) = stream.next().await {
                let content = materialize(req.id);
                tx.send(Ok(content)).await.unwrap();
            }
        });

        let stream = ReceiverStream::new(rx);
        Ok(Response::new(Box::pin(stream)))
    }
}

fn materialize(_id: u32) -> Content {
    Content::default()
}
