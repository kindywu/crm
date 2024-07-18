use futures::{Stream, StreamExt, TryStreamExt};
use sqlx::{PgPool, Pool, Postgres};
use std::pin::Pin;
use tonic::Status;
use user_stat::User;

pub type ResponseStream<'a> = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send + 'a>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let query = "SELECT email, name FROM user_stats LIMIT 3";
    let pool = PgPool::connect("postgres://kindy:kindy@localhost:5432/stats").await?;

    let app = App { pool };
    let request = Request {
        query: query.to_string(),
    };
    let mut users = app.raw_query(&request).await?;
    while let Some(user) = users.try_next().await? {
        println!("{:?}", user);
    }

    Ok(())
}

struct App {
    pool: Pool<Postgres>,
}

struct Request {
    query: String,
}

impl App {
    fn native_query<'a>(&'a self, req: &'a Request) -> ResponseStream<'a> {
        sqlx::query_as::<_, User>(&req.query)
            .fetch(&self.pool)
            .map_err(|e| Status::unknown(e.to_string()))
            .boxed()
    }

    async fn raw_query<'a>(&'a self, req: &'a Request) -> Result<ResponseStream<'a>, Status> {
        Ok(self.native_query(req))
    }
}
