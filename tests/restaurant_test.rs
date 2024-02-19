use sqlx::PgPool;
use uuid::Uuid;
use std::env;

extern crate rstnt_api;
use rstnt_api::restaurant::{create_items, get_item, get_items, delete_item, ItemCreate};
use rstnt_api::seed;

const TEST_TABLE_ID: i32 = 1;

// in order to ensure that the tests are isolated from each other, we will create a new database for each test
// recreate the test db container when you want to clean up the databases
async fn db_setup() -> PgPool {
    dotenv::dotenv().ok();
    let base_database_url = env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");
    
    let db = PgPool::connect(&base_database_url).await.expect("Failed to connect to the database");
    
    let test_database_name = Uuid::new_v4().to_string();
    let new_db_query = format!(r#"CREATE DATABASE "{}";"#, test_database_name);
    
    sqlx::query(&new_db_query).execute(&db).await.expect("Failed to create test database");

    let test_database_url = format!("{}/{}", base_database_url, test_database_name);
    let test_db = PgPool::connect(&test_database_url).await.expect("Failed to connect to the test database");

    sqlx::migrate!("./migrations")
    .run(&test_db)
    .await
    .expect("Failed to run migrations");

    seed::seed_tables_if_needed(&test_db, 100).await.expect("Failed to seed tables");

    test_db
}

#[tokio::test]
async fn test_create_items() {
    let db = db_setup().await;
    let table_id = 1; 
    let items = vec![ItemCreate { cook_time: 10 }];
    let result = create_items(&db, items, table_id).await.expect("failed to create items");
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_get_single_item() {
    let db = db_setup().await;
    
    let items_to_create = vec![ItemCreate { cook_time: 5 }];
    let created_items = create_items(&db, items_to_create, TEST_TABLE_ID)
        .await
        .expect("failed to create items for testing");
    
    let created_item_id = created_items.first().expect("no item was created").item_id;

    let item = get_item(&db, TEST_TABLE_ID, created_item_id)
        .await
        .expect("failed to retrieve the created item");
    assert_eq!(item.item_id, created_item_id);
}

#[tokio::test]
async fn test_get_multiple_items() {
    let db = db_setup().await;
    
    let items_to_create = vec![
        ItemCreate { cook_time: 5 },
        ItemCreate { cook_time: 10 },
        ItemCreate { cook_time: 15 },
    ];
    create_items(&db, items_to_create, TEST_TABLE_ID)
        .await
        .expect("failed to create items for testing");

    let retrieved_items = get_items(&db, TEST_TABLE_ID)
        .await
        .expect("failed to retrieve items");

    assert_eq!(retrieved_items.len(), 3, "did not retrieve the expected number of items");

    let cook_times: Vec<i32> = retrieved_items.into_iter().map(|item| item.cook_time).collect();
    assert!(cook_times.contains(&5) && cook_times.contains(&10) && cook_times.contains(&15), "retrieved items do not have the expected cook times");
}

#[tokio::test]
async fn test_delete_item() {
    let db = db_setup().await;
    
    let item_to_create = ItemCreate { cook_time: 5 };
    let created_items = create_items(&db, vec![item_to_create], TEST_TABLE_ID)
        .await
        .expect("failed to create item for testing");
    let created_item_id = created_items.first().expect("no item was created").item_id;

    delete_item(&db, TEST_TABLE_ID, created_item_id)
        .await
        .expect("failed to delete the item");

    let result = get_item(&db, TEST_TABLE_ID, created_item_id).await;

    assert!(result.is_err(), "the item should not exist after deletion");
}