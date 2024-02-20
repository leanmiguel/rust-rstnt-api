use rand::Rng;
use std::time::Duration;
use tokio::time;
use rstnt_api::restaurant::{ItemCreate, RestaurantItem};
use lazy_static::lazy_static;

lazy_static! {
    static ref PRINT_LOGS: bool = {
        let args: Vec<String> = std::env::args().collect();        
        args.contains(&"--print-logs".to_string())
    };
}

pub async fn spawn_clients() {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {            
        interval.tick().await;
        for _ in 0..10 {
            tokio::spawn(async {
                create_client().await.unwrap();
            });
        }
        
    }        
}
    
fn get_server_endpoint() -> String {
    let port = std::env::var("PORT").unwrap();
    format!("http://localhost:{}", port)
}

async fn create_client() -> Result<(), anyhow::Error> {   
    let create_items = call_create_items().await?;        
    let create_item = create_items.first();

    match create_item {
        Some(item) => {
            let table_id = item.table_id;            
            let item_id = item.item_id;
            
            call_get_items(table_id).await?;        
            call_get_item(table_id, item_id).await?;        
            call_delete_item(table_id, item_id).await?;
        
            if *PRINT_LOGS {
                println!("Item created and subsequently be deleted: {:?}", item);
            }
        }
        None => {
            println!("No items found");
        }
    }

    Ok(())
}

async fn call_get_items(table_id: i32) -> Result<Vec<RestaurantItem>, anyhow::Error> {
    let server_endpoint = get_server_endpoint();    
    let items = reqwest::get(format!("{}/api/tables/{}/items", server_endpoint, table_id)).await?.json::<Vec<RestaurantItem>>().await?;

    Ok(items)
}
#[derive(serde::Serialize)]
struct ItemsCreate {    
    items: Vec<ItemCreate>,
}

async fn call_create_items()-> Result<Vec<RestaurantItem>, anyhow::Error> {
    let server_endpoint = get_server_endpoint();    

    let body = ItemsCreate { items: vec![ItemCreate {cook_time: 5 }, ItemCreate {cook_time: 10 }] };    
    let table_id = Rng::gen_range(&mut rand::thread_rng(), 1..100);
    
    let client = reqwest::Client::new();
    let items = client.post(format!("{}/api/tables/{}/items", server_endpoint, table_id))
        .json(&body)
        .send()
        .await?
        .json::<Vec<RestaurantItem>>()
        .await?;

    Ok(items)    
}

async fn call_get_item(table_id: i32, item_id: i32) -> Result<RestaurantItem, anyhow::Error> {
    let server_endpoint = get_server_endpoint();
    let item = reqwest::get(format!("{}/api/tables/{}/items/{}", server_endpoint, table_id, item_id)).await?.json::<RestaurantItem>().await?;

    Ok(item)
}

async fn call_delete_item(table_id: i32, item_id: i32) -> Result<(), anyhow::Error> {
    let server_endpoint = get_server_endpoint();
    let client = reqwest::Client::new();    
    client.delete(format!("{}/api/tables/{}/items/{}", server_endpoint, table_id, item_id)).send().await?;

    Ok(())
}