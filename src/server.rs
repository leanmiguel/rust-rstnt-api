use axum::{Router, http::StatusCode, routing::get, response::IntoResponse}; 
use anyhow::Context;
use sqlx::PgPool;

use crate::handlers::restaurant_api_router;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn serve(db: PgPool) -> anyhow::Result<()> {
    let state = AppState { db };

    let app = Router::new()
    .nest("/api", restaurant_api_router())
    .route("/health", get(healthcheck_handler))
    .with_state(state);  

    let port = std::env::var("PORT").context("PORT is not set")?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.context("could not bind to address")?;
    axum::serve(listener, app).await.context("could not start server")?;
    
    Ok(())
}

pub async fn healthcheck_handler() -> impl IntoResponse {
    (StatusCode::OK, "ok").into_response()
}