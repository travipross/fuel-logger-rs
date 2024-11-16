use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Database(sqlx::Error),

    #[error("{0}")]
    Conversion(String),

    #[error["the resource was not found"]]
    ResourceNotFound,
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => ApiError::ResourceNotFound,
            _ => ApiError::Database(value),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            Self::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error".to_owned(),
            ),
            Self::Conversion(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "problem converting types".to_owned(),
            ),
            Self::ResourceNotFound => (StatusCode::NOT_FOUND, "resource not found".to_owned()),
        };

        (status, Json(json!({"error": msg}))).into_response()
    }
}
