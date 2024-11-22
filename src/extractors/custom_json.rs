use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
};

use crate::error::ApiError;

pub struct Json<T>(pub T);

#[async_trait]
impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                tracing::error!("problem extracting JSON request body: {rejection}");
                Err(ApiError::JsonError(rejection))
            }
        }
    }
}
