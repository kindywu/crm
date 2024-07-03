use chrono::{DateTime, TimeZone, Utc};
use itertools::Itertools;
use prost_types::Timestamp;
use std::pin::Pin;
use tokio_stream::{self as stream, Stream};

use tonic::{Response, Status};

use crate::{QueryRequest, RawQueryRequest, User, UserStatService};

pub type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

//  type QueryStream: Stream<Item = Result<User, Status>,> + Send+ 'static;

impl UserStatService {
    pub async fn query(&self, req: QueryRequest) -> Result<Response<ResponseStream>, Status> {
        let mut sql = "SELECT email, name FROM user_stats WHERE ".to_string();

        let time_conditions = req
            .timestamps
            .into_iter()
            .map(|(k, v)| timestamp_query(&k, v.lower, v.upper))
            .join(" AND ");

        sql.push_str(&time_conditions);

        let id_conditions = req
            .ids
            .into_iter()
            .map(|(k, v)| ids_query(&k, v.ids))
            .join(" AND ");

        sql.push_str(" AND ");
        sql.push_str(&id_conditions);

        println!("Generated SQL: {}", sql);

        self.raw_query(RawQueryRequest { query: sql }).await
    }

    pub async fn raw_query(
        &self,
        req: RawQueryRequest,
    ) -> Result<Response<ResponseStream>, Status> {
        let users = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.state.pool)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        let stream = stream::iter(users.into_iter().map(Ok));
        let response_stream = Box::pin(stream);
        Ok(Response::new(response_stream))
    }
}

fn timestamp_query(name: &str, lower: Option<Timestamp>, upper: Option<Timestamp>) -> String {
    if lower.is_none() && upper.is_none() {
        return "TRUE".to_string();
    }

    if lower.is_none() {
        let upper = ts_to_utc(upper.unwrap());
        return format!("{} <= '{}'", name, upper.to_rfc3339());
    }

    if upper.is_none() {
        let lower = ts_to_utc(lower.unwrap());
        return format!("{} >= '{}'", name, lower.to_rfc3339());
    }

    format!(
        "{} BETWEEN '{}' AND '{}'",
        name,
        ts_to_utc(upper.unwrap()).to_rfc3339(),
        ts_to_utc(lower.unwrap()).to_rfc3339()
    )
}

fn ts_to_utc(ts: Timestamp) -> DateTime<Utc> {
    Utc.timestamp_opt(ts.seconds, ts.nanos as _).unwrap()
}

fn ids_query(name: &str, ids: Vec<u32>) -> String {
    if ids.is_empty() {
        return "TRUE".to_string();
    }

    format!("array{:?} <@ {}", ids, name)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::Utc;
    use prost_types::Timestamp;
    use tokio_stream::StreamExt;

    use crate::{
        AppConfig, IdQuery, QueryRequestBuilder, RawQueryRequest, TimeQuery, UserStatService,
    };

    #[tokio::test]
    async fn query_should_work() -> Result<()> {
        let config = AppConfig::load().expect("Failed to load config");
        let svc = UserStatService::new(config).await?;
        let query = QueryRequestBuilder::default()
            .timestamp(("created_at".to_string(), tq(Some(120), None)))
            .timestamp(("last_visited_at".to_string(), tq(Some(30), None)))
            .id(("viewed_but_not_started".to_string(), id(&[252790])))
            .build()
            .unwrap();
        let mut stream = svc.query(query).await?.into_inner();

        while let Some(res) = stream.next().await {
            println!("{:?}", res);
        }
        Ok(())
    }
    fn id(id: &[u32]) -> IdQuery {
        IdQuery { ids: id.to_vec() }
    }

    fn tq(lower: Option<i64>, upper: Option<i64>) -> TimeQuery {
        TimeQuery {
            lower: lower.map(to_ts),
            upper: upper.map(to_ts),
        }
    }
    fn to_ts(days: i64) -> Timestamp {
        let dt = Utc::now()
            .checked_sub_signed(chrono::Duration::days(days))
            .unwrap();
        Timestamp {
            seconds: dt.timestamp(),
            nanos: dt.timestamp_subsec_nanos() as i32,
        }
    }

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
