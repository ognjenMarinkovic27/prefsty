use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("resource not found")]
    NotFound(sqlx::Error),

    #[error("conflict occured")]
    Conflict(sqlx::Error),

    #[error("referenced entity not found")]
    ForeignKeyViolation(sqlx::Error),

    #[error(transparent)]
    Sqlx(sqlx::Error),

    #[error("deserialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.is_unique_violation() {
                return DbError::Conflict(e);
            } else if db_err.is_foreign_key_violation() {
                return DbError::ForeignKeyViolation(e);
            }
        }

        DbError::Sqlx(e)
    }
}
