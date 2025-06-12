use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}
