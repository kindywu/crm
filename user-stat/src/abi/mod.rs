mod user_stat;

pub use user_stat::*;

use tonic::{Response, Status};

use crate::{user_stat_server::UserStat, QueryRequest, RawQueryRequest};

#[derive(Default)]
pub struct UserStatService {}

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
