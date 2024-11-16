use axum::{
    extract::Path,
    routing::{get, post, put},
    Json, Router,
};
use chrono::Utc;
use fake::{Fake, Faker};
use uuid::Uuid;

use crate::{
    types::log_record::{LogRecord, LogRecordInput},
    AppState,
};

async fn read(Path(log_record_id): Path<Uuid>) -> Json<LogRecord> {
    println!("Getting vehicle with ID: {log_record_id}");
    let log_record = Faker.fake::<LogRecord>();
    Json(log_record)
}

async fn list() -> Json<Vec<LogRecord>> {
    let log_records = Faker.fake::<Vec<LogRecord>>();
    Json(log_records)
}

async fn create(Json(log_record_input): Json<LogRecordInput>) -> Json<LogRecord> {
    let log_record = LogRecord {
        id: Faker.fake(),
        date: log_record_input.date.unwrap_or(Utc::now()),
        log_type: log_record_input.log_type,
        odometer: log_record_input.odometer,
    };
    println!("Created log record: {log_record:?}");
    Json(log_record)
}

async fn update(
    Path(log_record_id): Path<Uuid>,
    Json(log_record_input): Json<LogRecordInput>,
) -> Json<LogRecord> {
    let log_record = LogRecord {
        id: log_record_id,
        date: log_record_input.date.unwrap_or(Utc::now()),
        log_type: log_record_input.log_type,
        odometer: log_record_input.odometer,
    };
    println!("Created log record: {log_record:?}");
    Json(log_record)
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:log_record_id", get(read))
        .route("/:log_record_id", put(update))
}
