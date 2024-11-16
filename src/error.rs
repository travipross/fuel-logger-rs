use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),

    #[error("{0}")]
    Conversion(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "database error"),
            Self::Conversion(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "problem converting types",
            ),
        }
        .into_response()
    }
}
