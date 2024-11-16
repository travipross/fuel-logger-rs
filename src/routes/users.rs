use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    controllers::user::{
        create as create_user, delete as delete_user, list as list_users, read as read_user,
        update as update_user,
    },
    error::ApiError,
    types::user::{
        api::{
            CreateUserBody, CreateUserResponse, DeleteUserResponse, ListUsersResponse,
            ReadUserResponse, UpdateUserBody, UpdateUserResponse,
        },
        db::User as DbUser,
    },
    AppState,
};

async fn list(State(appstate): State<AppState>) -> Result<ListUsersResponse, ApiError> {
    list_users(&appstate.db).await
}

async fn read(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Result<ReadUserResponse, ApiError> {
    read_user(&appstate.db, user_id).await
}

async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateUserBody>,
) -> Result<CreateUserResponse, ApiError> {
    let db_user = DbUser::from_api_type(&Uuid::new_v4(), body);
    create_user(&appstate.db, db_user).await
}

async fn update(
    State(appstate): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<UpdateUserBody>,
) -> Result<UpdateUserResponse, ApiError> {
    let db_user = DbUser::from_api_type(&user_id, body);
    update_user(&appstate.db, user_id, db_user).await
}

async fn delete_route(
    Path(user_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Result<DeleteUserResponse, ApiError> {
    delete_user(&appstate.db, user_id).await
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:user_id", get(read))
        .route("/:user_id", put(update))
        .route("/:user_id", delete(delete_route))
}
