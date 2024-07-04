use crate::AppConfig;
use anyhow::Result;
use sqlx::PgPool;

pub struct AppState {
    pub config: AppConfig,
    pub pool: PgPool,
}

impl AppState {
    pub async fn try_new(config: AppConfig) -> Result<Self> {
        let pool = PgPool::connect(&config.server.db_url).await?;
        Ok(Self { config, pool })
    }
}

#[cfg(test)]
mod test_utils {
    use std::env;

    use anyhow::Result;
    use sqlx::{Pool, Postgres};
    use sqlx_db_tester::TestPg;

    use crate::{AppConfig, ServerConfig};

    use super::AppState;

    #[cfg(test)]
    impl AppState {
        pub async fn try_new_test() -> Result<(TestPg, Self)> {
            // read test db server
            let url = match env::var("DATABASE_URL") {
                Ok(url) => url,
                Err(_) => {
                    // 读取.env文件，读取数据库地址
                    dotenv::from_filename("./user-stat/.env").ok();
                    env::var("DATABASE_URL")?
                }
            };

            println!("test db server: {url}");
            // 初始化测试数据库
            let (tdb, _pool) = AppState::init_test_db(url).await?;
            println!("test db name: {}", tdb.dbname);

            let config = AppConfig {
                server: ServerConfig {
                    db_url: tdb.url(),
                    ..Default::default()
                },
            };

            Ok((tdb, Self::try_new(config).await?))
        }

        async fn init_test_db(url: String) -> Result<(TestPg, Pool<Postgres>)> {
            // 创建测试数据库
            // println!(
            //     "try to create test database. current dir is {:?} ",
            //     env::current_dir().unwrap()
            // );

            let migrations = std::path::Path::new("./migrations");
            // println!("migrations dir is {:?} ", migrations);

            let tdb = TestPg::new(url, migrations);
            // println!("tdb url is {:?} ", tdb.url());

            let pool = tdb.get_pool().await;

            // 插入准备数据
            let sqls: Vec<&str> = include_str!("../fixtures/test.sql").split(';').collect();

            if !sqls.is_empty() {
                let mut ts = pool.begin().await.expect("begin transaction failed");
                for sql in sqls {
                    if sql.trim().is_empty() {
                        continue;
                    }

                    // println!("sql: {sql}");

                    sqlx::query(sql)
                        .execute(&mut *ts)
                        .await
                        .expect("execute sql failed");
                }
                ts.commit().await.expect("commit transaction failed");
            }

            Ok((tdb, pool))
        }
    }
}
