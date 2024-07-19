use futures::{Stream, TryStreamExt};
use sqlx::{PgPool, Pool, Postgres};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Status;
use user_stat::User;

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

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
    fn native_query(&self, req: &Request) -> ResponseStream {
        let (tx, rx) = mpsc::channel(1024);

        let query = req.query.clone();
        let pool = self.pool.clone();

        tokio::spawn(async move {
            let mut stream = sqlx::query_as::<_, User>(&query)
                .fetch(&pool)
                .map_err(|e| Status::unknown(e.to_string()));

            while let Ok(Some(user)) = stream.try_next().await {
                tx.send(Ok(user)).await.unwrap()
            }
        });

        let stream = ReceiverStream::new(rx);

        Box::pin(stream)
    }

    async fn raw_query(&self, req: &Request) -> Result<ResponseStream, Status> {
        Ok(self.native_query(req))
    }
}
