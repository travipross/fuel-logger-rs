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
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "something went wrong"),
        }
        .into_response()
    }
}
