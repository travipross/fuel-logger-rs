use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::postgres::PgDatabaseError;

const POSTGRES_UNIQUE_VIOLATION: &str = "23505";

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Database(sqlx::Error),

    #[error("{0}")]
    Conversion(String),

    #[error("the resource was not found")]
    ResourceNotFound,

    #[error("the requested inputs violate a unique constraint")]
    UniqueConstraintViolation { detail: Option<String> },

    #[error("log record is of the wrong type and can't be updated")]
    WrongLogRecordType,

    #[error("{0}")]
    Configuration(#[from] config::ConfigError),

    #[error("{0}")]
    JsonError(#[from] JsonRejection),
}
impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        match &value {
            sqlx::Error::RowNotFound => ApiError::ResourceNotFound,
            sqlx::Error::Database(db_err) => {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    match pg_err.code() {
                        POSTGRES_UNIQUE_VIOLATION => ApiError::UniqueConstraintViolation {
                            detail: pg_err.detail().map(ToOwned::to_owned),
                        },
                        _ => ApiError::Database(value),
                    }
                } else {
                    ApiError::Database(value)
                }
            }
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
            Self::UniqueConstraintViolation { detail } => (
                StatusCode::CONFLICT,
                detail.unwrap_or("unknown violation".to_owned()),
            ),
            Self::WrongLogRecordType => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::Configuration(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "problem parsing configuration".to_owned(),
            ),
            Self::JsonError(e) => (e.status(), e.body_text()),
        };

        (
            status,
            Json(json!({"error_msg": msg, "code": status.as_u16()})),
        )
            .into_response()
    }
}
