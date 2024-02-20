use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    AnyhowError (#[from] anyhow::Error),
}