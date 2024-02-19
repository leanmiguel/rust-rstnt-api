use crate::server::AppState;
use crate::restaurant::{create_items, get_item, get_items, delete_item, ItemCreate, RestaurantItem};

use axum::extract::{State, Path};
use axum::{Json, http::StatusCode, response::IntoResponse}; 
use axum::{Router, routing::get};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ItemsCreate {    
    items: Vec<ItemCreate>,
}

pub async fn delete_item_handler(State(state): State<AppState>, Path((table_id, item_id)): Path<(i32, i32)>) -> impl IntoResponse {
    match delete_item(&state.db, table_id, item_id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_item_handler(State(state): State<AppState>, Path((table_id, item_id)): Path<(i32, i32)>) -> impl IntoResponse {
    match get_item(&state.db, table_id, item_id).await {
        Ok(res) => (StatusCode::OK, Json(RestaurantItem::from(res))).into_response(),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            match e {
                // sqlx::Error::RowNotFound => return StatusCode::NOT_FOUND.into_response(),
                _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            }
        }
    }
}

pub async fn get_items_handler(State(state): State<AppState>, Path(table_id): Path<i32>) -> impl IntoResponse {
    match get_items(&state.db, table_id).await {
        Ok(res) => (StatusCode::OK, Json(res.into_iter().map(RestaurantItem::from).collect::<Vec<RestaurantItem>>())).into_response(),
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}


pub async fn create_items_handler(State(state): State<AppState>, Path(table_id): Path<i32>, Json(payload): Json<ItemsCreate>) -> impl IntoResponse {     
    match create_items(&state.db, payload.items, table_id).await {
        Ok(res) => (StatusCode::OK, Json(res.into_iter().map(RestaurantItem::from).collect::<Vec<_>>())).into_response(),
        Err(e) => {
            // let pg_error = e.as_database_error();
            // match pg_error.and_then(|e| e.code()) {
            //     Some(code) if code == "23503" => {
            //         eprintln!("Database error: {:?}", e);
            //         (StatusCode::BAD_REQUEST, "Foreign key constraint violation: The specified table_id does not exist.").into_response()
            //     },
            //     _ => {
            //         eprintln!("Database error: {:?}", e);
            //         StatusCode::INTERNAL_SERVER_ERROR.into_response()
            //     }
            // }
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }     
    }
}

pub fn restaurant_api_router() -> Router<AppState> {
    return Router::new()
        .route("/tables/:table_id/items", get(get_items_handler).post(create_items_handler))    
        .route("/tables/:table_id/items/:id", get(get_item_handler).delete(delete_item_handler));
} 