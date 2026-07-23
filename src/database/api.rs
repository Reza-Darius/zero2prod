use anyhow::Context;
use anyhow::Result;
use sqlx::{PgPool, Pool, Postgres, migrate::Migrator, postgres::PgPoolOptions};
use tracing::debug;

use crate::models::User;

static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

#[derive(Debug)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    pub async fn new(url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await
            .with_context(|| format!("failed to connect to {}", url))?;

        MIGRATOR
            .run(&pool)
            .await
            .with_context(|| "migration failed")?;

        Ok(Database { pool })
    }

    pub async fn new_sub(&self, user_name: &str, email: &str) -> Result<()> {
        sqlx::query("INSERT INTO subscriptions VALUES(?, ?)")
            .bind(user_name)
            .bind(email)
            .execute(&self.pool)
            .await?;

        debug!(user_name, email, "added new sub");
        Ok(())
    }

    pub async fn get_subs(&self) -> Result<Vec<User>> {
        let res = sqlx::query_as::<_, User>("SELECT * FROM subscriptions")
            .fetch_all(&self.pool)
            .await?;

        debug!("fetched users");
        Ok(res)
    }
}

impl From<PgPool> for Database {
    fn from(value: PgPool) -> Self {
        Database { pool: value }
    }
}
