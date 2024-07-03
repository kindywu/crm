mod user_stat;
use anyhow::Result;
use std::sync::Arc;

use sqlx::PgPool;
pub use user_stat::*;

use tonic::{Response, Status};

use crate::{user_stat_server::UserStat, AppConfig, QueryRequest, RawQueryRequest};

#[allow(unused)]
pub struct UserStatServiceState {
    config: AppConfig,
    pool: PgPool,
}

pub struct UserStatService {
    state: Arc<UserStatServiceState>,
}

impl UserStatService {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self {
            state: Arc::new(UserStatServiceState { config, pool }),
        })
    }
}

#[tonic::async_trait]
impl UserStat for UserStatService {
    type QueryStream = ResponseStream;

    async fn query(
        &self,
        request: tonic::Request<QueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        let request = request.into_inner();
        self.query(request).await
    }

    type RawQueryStream = ResponseStream;

    async fn raw_query(
        &self,
        request: tonic::Request<RawQueryRequest>,
    ) -> Result<Response<Self::QueryStream>, Status> {
        let request = request.into_inner();
        self.raw_query(request).await
    }
}
