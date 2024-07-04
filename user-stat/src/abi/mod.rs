mod user_stat;
use anyhow::Result;
use std::sync::Arc;

pub use user_stat::*;

use tonic::{Response, Status};

use crate::{app_state::AppState, user_stat_server::UserStat, QueryRequest, RawQueryRequest};

pub struct UserStatService {
    state: Arc<AppState>,
}

impl UserStatService {
    pub async fn new(app_state: AppState) -> Result<Self> {
        Ok(Self {
            state: Arc::new(app_state),
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
