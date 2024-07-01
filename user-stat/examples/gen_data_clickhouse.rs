use anyhow::Result;
use clickhouse::{Client, Row};
use fake::{
    faker::internet::en::SafeEmail, faker::name::zh_cn::Name, faker::time::en::DateTimeBetween,
    Dummy, Fake, Faker,
};
use nanoid::nanoid;
use rand::Rng;
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use time::{ext::NumericalDuration, OffsetDateTime};
use tokio::time::Instant;

/*
SELECT count(*) FROM user_stats

SELECT
    sum(rows) AS `总行数`,
    formatReadableSize(sum(data_uncompressed_bytes)) AS `原始大小`,
    formatReadableSize(sum(data_compressed_bytes)) AS `压缩大小`,
    round((sum(data_compressed_bytes) / sum(data_uncompressed_bytes)) * 100, 0) AS `压缩率`
FROM system.parts

SELECT
    table AS `表名`,
    sum(rows) AS `总行数`,
    formatReadableSize(sum(data_uncompressed_bytes)) AS `原始大小`,
    formatReadableSize(sum(data_compressed_bytes)) AS `压缩大小`,
    round((sum(data_compressed_bytes) / sum(data_uncompressed_bytes)) * 100, 0) AS `压缩率`
FROM system.parts
WHERE table IN ('user_stats')
GROUP BY table

*/
#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default()
        .with_url("http://localhost:8123")
        .with_database("stats");

    for i in 0..50 {
        let mut insert = client.insert("user_stats")?;
        print!("batch insert {} started. ", i);
        let begin = Instant::now();
        for _ in 0..100_000 {
            let user_stat = Faker.fake::<UserStat>();
            insert.write(&user_stat).await?;
        }
        insert.end().await?;
        println!("finish in {:?}", begin.elapsed());
    }

    println!("Inserted 10 records successfully!");

    Ok(())
}

#[derive(Debug, Clone, Dummy, Serialize, Row, PartialEq, Eq)]
struct UserStat {
    #[dummy(faker = "UniqueEmail")]
    email: String,
    #[dummy(faker = "Name()")]
    name: String,
    gender: Gender,
    #[dummy(faker = "DateTimeBetween(before(365*5), before(90))")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    created_at: OffsetDateTime,
    #[dummy(faker = "DateTimeBetween(before(30), now())")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    last_visited_at: OffsetDateTime,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    last_watched_at: OffsetDateTime,
    #[dummy(faker = "IntList(50, 100000, 100000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntList(50, 200000, 100000)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntList(50, 300000, 100000)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntList(50, 400000, 100000)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(before(45), now())")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    last_email_notification: OffsetDateTime,
    #[dummy(faker = "DateTimeBetween(before(15), now())")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    last_in_app_notification: OffsetDateTime,
    #[dummy(faker = "DateTimeBetween(before(90), now())")]
    #[serde(with = "clickhouse::serde::time::datetime")]
    last_sms_notification: OffsetDateTime,
}

#[derive(Debug, Clone, Dummy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(u8)]
enum Gender {
    Female = 2,
    Male = 1,
    Unknown = 0,
}

struct UniqueEmail;

const ALPHABET: [char; 36] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UniqueEmail, rng: &mut R) -> String {
        let email: String = SafeEmail().fake_with_rng(rng);
        let id = nanoid!(8, &ALPHABET);
        let at = email.find('@').unwrap();
        format!("{}.{}{}", &email[..at], id, &email[at..])
    }
}

fn before(days: i64) -> OffsetDateTime {
    OffsetDateTime::now_utc().checked_sub(days.days()).unwrap()
}

fn now() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

struct IntList(pub i32, pub i32, pub i32);

impl Dummy<IntList> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(v: &IntList, rng: &mut R) -> Vec<i32> {
        let (max, start, len) = (v.0, v.1, v.2);
        let size = rng.gen_range(0..max);
        (0..size)
            .map(|_| rng.gen_range(start..start + len))
            .collect()
    }
}
