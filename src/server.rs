use axum::Router;
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
    .with_state(state);  

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8020").await.context("could not bind to address")?;
    axum::serve(listener, app).await.context("could not start server")?;

    Ok(())
}