use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    controllers::user as controller,
    error::ApiError,
    models::api::{
        CreateUserBody, CreateUserResponse, DeleteUserResponse, ListUsersResponse,
        ReadUserResponse, UpdateUserBody, UpdateUserResponse,
    },
    AppState,
};

async fn list(State(appstate): State<AppState>) -> Result<ListUsersResponse, ApiError> {
    controller::list(&appstate.db).await
}

async fn read(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<ReadUserResponse, ApiError> {
    controller::read(&appstate.db, &user_id).await
}

async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<CreateUserResponse, ApiError> {
    controller::create(&appstate.db, body).await
}

async fn update(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UpdateUserBody>,
) -> Result<UpdateUserResponse, ApiError> {
    controller::update(&appstate.db, &user_id, body).await
}

async fn delete_route(
    Path(user_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Result<DeleteUserResponse, ApiError> {
    controller::delete(&appstate.db, &user_id).await
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:user_id", get(read))
        .route("/:user_id", put(update))
        .route("/:user_id", delete(delete_route))
}

#[cfg(test)]
mod router_tests {
    use super::*;
    use serde_json::json;
    use tower::ServiceExt;

    mod create {
        use std::usize;

        use axum::{
            body::Body,
            http::{Method, Request, StatusCode},
        };
        use fake::{faker, Fake};
        use sqlx::PgPool;

        use super::*;

        #[sqlx::test]
        async fn can_create_record_with_valid_inputs(pool: PgPool) {
            // Arrange
            let router = build_router().with_state(AppState { db: pool });

            let first_name = faker::name::en::FirstName().fake::<String>();
            let last_name = faker::name::en::LastName().fake::<String>();
            let username = faker::internet::en::Username().fake::<String>();
            let email = faker::internet::en::FreeEmail().fake::<String>();

            let input = json!({
                "first_name": first_name,
                "last_name": last_name,
                "username": username,
                "email": email,
            });

            // Act
            let req = Request::builder()
                .uri("/")
                .method(Method::POST)
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&input).expect("could not form request"),
                ))
                .expect("could not build request");

            let res = router.oneshot(req).await.expect("could not be called");

            // Assert
            assert_eq!(res.status(), StatusCode::CREATED);
            let body = axum::body::to_bytes(res.into_body(), usize::MAX)
                .await
                .expect("could not convert to bytes");
            let body_json =
                serde_json::from_slice::<serde_json::Value>(&body).expect("could not extract");
            let body_obj = body_json
                .as_object()
                .expect("could not convert to object")
                .clone();

            assert_eq!(body_obj.keys().len(), 1);
            assert!(body_obj
                .keys()
                .collect::<Vec<_>>()
                .contains(&&"id".to_owned()));
            assert!(body_obj.get("id").is_some());
        }
    }
}
