use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    controllers::log_record as controller,
    error::ApiError,
    models::{
        api::{
            CreateLogRecordBody, CreateLogRecordResponse, DeleteLogRecordResponse,
            ListLogRecordsResponse, ReadLogRecordResponse, UpdateLogRecordBody,
            UpdateLogRecordResponse,
        },
        db::LogRecord as DbLogRecord,
    },
    AppState,
};

async fn read(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
) -> Result<ReadLogRecordResponse, ApiError> {
    controller::read(&appstate.db, &log_record_id).await
}

async fn list(State(appstate): State<AppState>) -> Result<ListLogRecordsResponse, ApiError> {
    controller::list(&appstate.db).await
}

async fn create(
    State(appstate): State<AppState>,
    Json(log_record_input): Json<CreateLogRecordBody>,
) -> Result<CreateLogRecordResponse, ApiError> {
    let db_log_record = DbLogRecord::from_api_type(&uuid::Uuid::new_v4(), log_record_input);
    controller::create(&appstate.db, db_log_record).await
}

async fn update(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
    Json(log_record_input): Json<UpdateLogRecordBody>,
) -> Result<UpdateLogRecordResponse, ApiError> {
    let db_log_record = DbLogRecord::from_api_type(&log_record_id, log_record_input);
    controller::update(&appstate.db, &log_record_id, db_log_record).await
}

async fn delete_route(
    State(appstate): State<AppState>,
    Path(log_record_id): Path<Uuid>,
) -> Result<DeleteLogRecordResponse, ApiError> {
    controller::delete(&appstate.db, &log_record_id).await
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:log_record_id", get(read))
        .route("/:log_record_id", put(update))
        .route("/:log_record_id", delete(delete_route))
}
