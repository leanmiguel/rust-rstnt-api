use anyhow::{Result, Context};
use dotenv::dotenv;

mod db;
mod server;
mod handlers;
mod client;

use rstnt_api::restaurant;
use rstnt_api::seed;

const SEED_TABLE_COUNT: i32 = 100;

#[tokio::main]
async fn main() -> Result<()>{    
    dotenv().ok(); // don't panic if it doesn't properly load, as .env is optional for deployment
    
    let db = db::new_db().await.context("could not connect to the database")?;
    
    seed::seed_tables_if_needed(&db, SEED_TABLE_COUNT).await.context("could not seed tables")?;
    
    let cli_args: Vec<String> = std::env::args().collect();
    let enable_client = cli_args.contains(&"--enable_client".to_string());

    if enable_client {
        tokio::spawn(async {server::serve(db).await.unwrap()});
        client::spawn_clients().await;        
    } else {
        server::serve(db).await.context("could not start server")?;
    }

    Ok(())
}