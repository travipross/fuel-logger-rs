use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

use crate::{
    controllers::user as controller,
    error::ApiError,
    extractors::custom_json::Json,
    models::api::{
        CreateUserBody, CreateUserResponse, DeleteUserResponse, ListUsersResponse,
        ReadUserResponse, UpdateUserBody, UpdateUserResponse,
    },
    AppState,
};

#[tracing::instrument(name = "users_list_route", skip(appstate), err)]
async fn list(State(appstate): State<AppState>) -> Result<ListUsersResponse, ApiError> {
    controller::list(&appstate.db).await
}

#[tracing::instrument(name = "users_read_route", skip(appstate), err)]
async fn read(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<ReadUserResponse, ApiError> {
    controller::read(&appstate.db, &user_id).await
}

#[tracing::instrument(name = "users_create_route", skip(appstate), err)]
async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<CreateUserResponse, ApiError> {
    controller::create(&appstate.db, body).await
}

#[tracing::instrument(name = "users_update_route", skip(appstate), err)]
async fn update(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UpdateUserBody>,
) -> Result<UpdateUserResponse, ApiError> {
    controller::update(&appstate.db, &user_id, body).await
}

#[tracing::instrument(name = "users_delete_route", skip(appstate), err)]
async fn delete_route(
    Path(user_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Result<DeleteUserResponse, ApiError> {
    controller::delete(&appstate.db, &user_id).await
}

#[tracing::instrument(name = "build_users_router", skip_all)]
pub fn build_router() -> Router<AppState> {
    tracing::debug!("building users router");
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:user_id", get(read))
        .route("/:user_id", put(update))
        .route("/:user_id", delete(delete_route))
}
