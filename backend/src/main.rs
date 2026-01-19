mod db;
use dotenv::dotenv;
use std::env;

use crate::db::*;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .connect(
            env::var("DATABASE_URL")
                .expect("DATABASE_URL environment variable must be set")
                .as_str(),
        )
        .await?;
    let mut conn = pool.acquire().await?;

    return Ok(());
}
