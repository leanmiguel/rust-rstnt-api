use anyhow::{Context, Error};
use sqlx::PgPool;
pub async fn seed_tables_if_needed(db: &PgPool, max_table_count: i32) -> Result<(), Error> {
    let current_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM restaurant_tables")
        .fetch_one(db)
        .await?;
    
    let rows_to_add = max_table_count - current_count as i32;

    // this needs to loop because there's no way to insert multiple rows at once for a table with only an identity column
    // this shouldn't happen in a real application, as the restaurant_table should have other columns
    if rows_to_add > 0 {
        for _i in 0..rows_to_add {
            sqlx::query("INSERT INTO restaurant_tables DEFAULT VALUES").execute(db).await.context("could not insert into restaurant_tables")?;
        }
    }

    Ok(())
}
