use chrono::{Duration, Utc};
use prost_types::Timestamp;
use user_stat::{QueryRequest, QueryRequestBuilder, RawQueryRequest, TimeQuery};

pub fn new_query_request_for_wellcome(interval: i64) -> QueryRequest {
    let name = "created_at";
    let d1 = Utc::now() - Duration::days(interval);
    let d2 = d1 + Duration::days(1);

    let d1 = Timestamp {
        seconds: d1.timestamp(),
        nanos: 0,
    };

    let d2 = Timestamp {
        seconds: d2.timestamp(),
        nanos: 0,
    };

    QueryRequestBuilder::default()
        .timestamp((
            name.into(),
            TimeQuery {
                lower: Some(d1),
                upper: Some(d2),
            },
        ))
        .build()
        .expect("Failed to build query request")
}

pub fn new_raw_query_request_for_recall(interval: i64) -> RawQueryRequest {
    let lower = Utc::now() - Duration::days(interval);
    let upper = lower + Duration::days(1); // 只取一天的数据

    let lower = Timestamp {
        seconds: lower.timestamp(),
        nanos: 0,
    };

    let upper = Timestamp {
        seconds: upper.timestamp(),
        nanos: 0,
    };

    // 最后一次访问后，还没有被提醒过，并且最后一次访问时间在now-interval这一天内
    let sql =
        format!("SELECT email, name FROM user_stats WHERE last_visited_at > last_email_notification AND last_visited_at BETWEEN '{}' AND '{}'", lower, upper);
    println!("Generated SQL: {}", sql);

    RawQueryRequest { query: sql }
}

pub fn new_raw_query_request_for_remind(interval: i64) -> RawQueryRequest {
    let lower = Utc::now() - Duration::days(interval);
    let upper = lower + Duration::days(1); // 只取一天的数据

    let lower = Timestamp {
        seconds: lower.timestamp(),
        nanos: 0,
    };

    let upper = Timestamp {
        seconds: upper.timestamp(),
        nanos: 0,
    };

    // 最后一次查看在interval这一天，且viewed_but_not_started和started_but_not_finished为空数组
    let sql = format!(
        "SELECT email, name,
        viewed_but_not_started as viewed_but_not_started,
        started_but_not_finished as started_but_not_finisheds
        FROM user_stats WHERE last_watched_at BETWEEN '{}' AND '{}'
        AND viewed_but_not_started IS NOT NULL AND array_length(viewed_but_not_started, 1) > 0
        AND started_but_not_finished IS NOT NULL AND array_length(started_but_not_finished, 1) > 0",
        lower, upper
    );
    println!("Generated SQL: {}", sql);

    RawQueryRequest { query: sql }
}
