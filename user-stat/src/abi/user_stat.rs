use std::pin::Pin;

use tokio_stream::{self as stream, Stream};

use tonic::{Response, Status};

use crate::{QueryRequest, RawQueryRequest, User, UserStatService};

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

impl UserStatService {
    pub async fn query(&self, _request: QueryRequest) -> Result<Response<ResponseStream>, Status> {
        todo!()
    }

    pub async fn raw_query(
        &self,
        req: RawQueryRequest,
    ) -> Result<Response<ResponseStream>, Status> {
        let result: Result<Vec<User>, sqlx::Error> =
            sqlx::query_as(&req.query).fetch_all(&self.state.pool).await;

        match result {
            Ok(users) => {
                let stream = stream::iter(users.into_iter().map(Ok));
                let response_stream = Box::pin(stream);
                Ok(Response::new(response_stream))
            }
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }
}
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tokio_stream::StreamExt;

    use crate::{AppConfig, RawQueryRequest, UserStatService};

    #[tokio::test]
    async fn raw_query_should_work() -> Result<()> {
        let config = AppConfig::load().expect("Failed to load config");
        let svc = UserStatService::new(config).await?;

        let query = "select * from user_stats where created_at > '2024-01-01' limit 5".to_string();

        let mut stream = svc.raw_query(RawQueryRequest { query }).await?.into_inner();

        while let Some(Ok(res)) = stream.next().await {
            println!("{:?}", res);
        }
        Ok(())
    }
}
