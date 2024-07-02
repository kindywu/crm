use std::pin::Pin;

use tokio_stream::Stream;
use tonic::{Response, Status};

use crate::{QueryRequest, RawQueryRequest, User, UserStatService};

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

impl UserStatService {
    pub async fn query(&self, _request: QueryRequest) -> Result<Response<ResponseStream>, Status> {
        todo!()
    }

    pub async fn raw_query(
        &self,
        _request: RawQueryRequest,
    ) -> Result<Response<ResponseStream>, Status> {
        todo!()
    }
}
