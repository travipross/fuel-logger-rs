use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

use crate::{
    controllers::log_record as controller,
    error::ApiError,
    extractors::custom_json::Json,
    models::api::{
        CreateLogRecordBody, CreateLogRecordResponse, DeleteLogRecordResponse,
        ListLogRecordsResponse, ReadLogRecordResponse, UpdateLogRecordBody,
        UpdateLogRecordResponse,
    },
    AppState,
};

#[tracing::instrument(name = "log_records_read_route", skip(appstate), err)]
async fn read(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
) -> Result<ReadLogRecordResponse, ApiError> {
    controller::read(&appstate.db, &log_record_id).await
}

#[tracing::instrument(name = "log_records_list_route", skip(appstate), err)]
async fn list(State(appstate): State<AppState>) -> Result<ListLogRecordsResponse, ApiError> {
    controller::list(&appstate.db).await
}

#[tracing::instrument(name = "log_records_create_route", skip(appstate), err)]
async fn create(
    State(appstate): State<AppState>,
    Json(log_record_input): Json<CreateLogRecordBody>,
) -> Result<CreateLogRecordResponse, ApiError> {
    controller::create(&appstate.db, log_record_input).await
}

#[tracing::instrument(name = "log_records_update_route", skip(appstate), err)]
async fn update(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
    Json(log_record_input): Json<UpdateLogRecordBody>,
) -> Result<UpdateLogRecordResponse, ApiError> {
    controller::update(&appstate.db, &log_record_id, log_record_input).await
}

#[tracing::instrument(name = "log_records_delete_route", skip(appstate), err)]
async fn delete_route(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
) -> Result<DeleteLogRecordResponse, ApiError> {
    controller::delete(&appstate.db, &log_record_id).await
}

#[tracing::instrument(name = "build_log_records_router", skip_all)]
pub fn build_router() -> Router<AppState> {
    tracing::debug!("building log_records router");
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:log_record_id", get(read))
        .route("/:log_record_id", put(update))
        .route("/:log_record_id", delete(delete_route))
}
