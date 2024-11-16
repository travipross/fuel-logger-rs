use crate::{error::ApiError, models::db::LogRecord as DbLogRecord, types::log_type::LogType};
use axum::{http::StatusCode, response::IntoResponse, Json};
use uuid::Uuid;

// Create
#[derive(Debug, PartialEq, serde::Deserialize, fake::Dummy)]
pub struct CreateLogRecordBody {
    pub date: Option<chrono::DateTime<chrono::Utc>>,
    pub vehicle_id: Uuid,
    #[serde(flatten)]
    pub log_type: LogType,
    #[dummy(faker = "0..500000")]
    pub odometer: u16,
    pub notes: Option<String>,
}

#[derive(Debug, serde::Serialize, fake::Dummy)]
pub struct CreateLogRecordResponse {
    pub id: Uuid,
}

impl IntoResponse for CreateLogRecordResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            [("location", format!("/log_records/{}", self.id))],
            Json(self),
        )
            .into_response()
    }
}

// Read
#[derive(Debug, PartialEq, serde::Serialize, fake::Dummy)]
pub struct ReadLogRecordResponse {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub date: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub log_type: LogType,
    #[dummy(faker = "0..500000")]
    pub odometer: u16,
    pub notes: Option<String>,
}

impl IntoResponse for ReadLogRecordResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// List
#[derive(Debug, serde::Serialize, fake::Dummy)]
pub struct ListLogRecordsResponse(Vec<ReadLogRecordResponse>);

impl IntoResponse for ListLogRecordsResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl TryFrom<DbLogRecord> for ReadLogRecordResponse {
    type Error = ApiError;
    fn try_from(value: DbLogRecord) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            vehicle_id: value.vehicle_id,
            date: value.date,
            log_type: value.log_type,
            odometer: u16::try_from(value.odometer)
                .map_err(|e| ApiError::Conversion(e.to_string()))?,
            notes: value.notes,
        })
    }
}

impl FromIterator<ReadLogRecordResponse> for ListLogRecordsResponse {
    fn from_iter<T: IntoIterator<Item = ReadLogRecordResponse>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

// Update
pub type UpdateLogRecordBody = CreateLogRecordBody;
pub type UpdateLogRecordResponse = ReadLogRecordResponse;

// Delete
#[derive(Debug, serde::Serialize, fake::Dummy)]
pub struct DeleteLogRecordResponse;

impl IntoResponse for DeleteLogRecordResponse {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::NO_CONTENT).into_response()
    }
}
