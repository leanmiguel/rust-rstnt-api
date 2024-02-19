use anyhow::{Result, Context};
use dotenv::dotenv;

mod db;
mod server;
mod handlers;
use rstnt_api::restaurant;
use rstnt_api::seed;

const SEED_TABLE_COUNT: i32 = 100;

#[tokio::main]
async fn main() -> Result<()>{    
    dotenv()?;    
    let db = db::new_db().await.context("could not connect to the database")?;
    seed::seed_tables_if_needed(&db, SEED_TABLE_COUNT).await.context("could not seed tables")?;
    server::serve(db).await.context("could not start server")?;

    Ok(())
}