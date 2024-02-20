use crate::models::Item;
use crate::error::Error;

use anyhow::anyhow;
use sqlx::{postgres::Postgres, Pool, QueryBuilder};
use serde::{Deserialize, Serialize};

#[derive(Debug,Clone,Deserialize)]
pub struct ItemCreate {
    pub cook_time: i32,
}

impl ItemCreate{
    fn validate(&self) -> Result<(), &'static str>{
        match self.cook_time {
            cook_time if cook_time > 0 => Ok(()),
            _ => Err("cook time must be greater than 0."),        
        }
    }
}

#[derive(Serialize)]
pub struct RestaurantItem {
    pub item_id: i32,
    pub table_id: i32,
    pub cook_time: i32,   
}

impl From<Item> for RestaurantItem {
    fn from(item: Item) -> Self {
        RestaurantItem {
            item_id: item.item_id,
            table_id: item.table_id,
            cook_time: item.cook_time,
        }
    }
}

pub async fn delete_item(db: &Pool<Postgres>, table_id: i32, item_id: i32) -> Result<(), Error> {
    sqlx::query!("DELETE FROM table_items WHERE table_id = $1 AND item_id = $2", table_id, item_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn get_item(db: &Pool<Postgres>, table_id: i32, item_id: i32) -> Result<Item, Error> {
    let result = sqlx::query_as!(Item, "SELECT * FROM table_items WHERE table_id = $1 AND item_id = $2", table_id, item_id)
        .fetch_one(db)
        .await?;

    Ok(result)
}

pub async fn get_items(db: &Pool<Postgres>, table_id: i32) -> Result<Vec<Item>, Error> { 
    let result = sqlx::query_as!(Item, "SELECT * FROM table_items WHERE table_id = $1", table_id)
        .fetch_all(db)
        .await?;

    Ok(result)
}

pub async fn create_items(db: &Pool<Postgres>, items: Vec<ItemCreate>, table_id: i32) -> Result<Vec<Item>, Error> {
    let invalid_items: Vec<ItemCreate> = items.iter()
    .filter(|item| item.validate().is_err())
    .cloned()
    .collect();
    
    if !invalid_items.is_empty() {
        return Err(Error::AnyhowError(anyhow!("Invalid items: {:?}", invalid_items)));
    }

    let mut query_builder = QueryBuilder::<Postgres>::new("INSERT INTO table_items (table_id, cook_time) ");
    query_builder.push_values(items, |mut b, new_item| {    
        b.push_bind(table_id).push_bind(new_item.cook_time);
    });
    query_builder.push(" RETURNING *");
    let query = query_builder.build_query_as::<Item>();

    let result = query.fetch_all(db).await?;

    Ok(result)
}