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