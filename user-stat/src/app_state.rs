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
