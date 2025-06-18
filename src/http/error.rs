use axum::{http::StatusCode, response::IntoResponse};

use crate::http::repos::error::DbError;

pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Unexpected error occurred: {}", err),
        }
    }
}

impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound(_) => AppError::new(StatusCode::NOT_FOUND, err.to_string()),
            DbError::Conflict(_) => AppError::new(StatusCode::CONFLICT, err.to_string()),
            DbError::ForeignKeyViolation(_) => {
                AppError::new(StatusCode::BAD_REQUEST, err.to_string())
            }
            _ => AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unexpected error occurred: {}", err),
            ),
        }
    }
}
