use sqlx::postgres::{PgPoolOptions, PgPool};
use anyhow::Context;


pub async fn new_db() -> anyhow::Result<PgPool> {
    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    
    let db = PgPoolOptions::new()
    .max_connections(10)
    .connect(&db_url)
    .await
    .context("could not connect to the database")?;

    Ok(db)
}
