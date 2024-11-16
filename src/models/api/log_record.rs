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

#[cfg(test)]
mod serde_tests {
    use super::*;
    use crate::utils::test_utils::merge_json_objects;
    use fake::{Fake, Faker};
    use serde_json::json;

    mod create {
        use super::*;
        mod request {
            use super::*;

            #[test]
            fn deserializes_correctly() {
                // Arrange
                let log_type = Faker.fake::<LogType>();
                let notes = Faker.fake::<Option<String>>();
                let expected = CreateLogRecordBody {
                    log_type: log_type.clone(),
                    notes: notes.clone(),
                    ..Faker.fake()
                };

                let mut json = json!({
                    "vehicle_id": expected.vehicle_id,
                    "odometer": expected.odometer,
                    "date": expected.date,
                });

                if let Some(notes) = notes {
                    merge_json_objects(&mut json, json!({"notes": notes}))
                }

                merge_json_objects(&mut json, json!(log_type));

                // Act
                let deserialized = serde_json::from_value::<CreateLogRecordBody>(json)
                    .expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }
        }

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let sample_record = Faker.fake::<CreateLogRecordResponse>();
                let expected = json!({
                    "id": sample_record.id
                });

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }

    mod read {
        use super::*;

        mod response {

            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let log_type = Faker.fake::<LogType>();
                let notes = Faker.fake::<Option<String>>();

                let sample_record = ReadLogRecordResponse {
                    log_type: log_type.clone(),
                    notes: notes.clone(),
                    ..Faker.fake()
                };

                let mut expected = json!({
                    "id": sample_record.id,
                    "vehicle_id": sample_record.vehicle_id,
                    "odometer": sample_record.odometer,
                    "date": sample_record.date,
                    "notes": notes,
                });
                merge_json_objects(&mut expected, json!(log_type));

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }

    mod list {
        use super::*;

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let sample_records = Faker.fake::<Vec<ReadLogRecordResponse>>();

                // Act
                let serialized =
                    serde_json::to_value(&sample_records).expect("could not serialize");

                // Assert
                let mut record_value_array = vec![];
                for record in sample_records {
                    record_value_array.push(
                        serde_json::to_value(record)
                            .expect("could not serialize individual record"),
                    );
                }

                assert_eq!(json!(record_value_array), serialized)
            }
        }
    }

    mod update {
        use super::*;

        mod request {
            use super::*;

            #[test]
            fn deserializes_correctly() {
                // Arrange
                let log_type = Faker.fake::<LogType>();
                let notes = Faker.fake::<Option<String>>();
                let expected = UpdateLogRecordBody {
                    log_type: log_type.clone(),
                    notes: notes.clone(),
                    ..Faker.fake()
                };

                let mut json = json!({
                    "vehicle_id": expected.vehicle_id,
                    "odometer": expected.odometer,
                    "date": expected.date,
                });

                if let Some(notes) = notes {
                    merge_json_objects(&mut json, json!({"notes": notes}))
                }

                merge_json_objects(&mut json, json!(log_type));

                // Act
                let deserialized = serde_json::from_value::<UpdateLogRecordBody>(json)
                    .expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }
        }

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let log_type = Faker.fake::<LogType>();
                let notes = Faker.fake::<Option<String>>();

                let sample_record = UpdateLogRecordResponse {
                    log_type: log_type.clone(),
                    notes: notes.clone(),
                    ..Faker.fake()
                };

                let mut expected = json!({
                    "id": sample_record.id,
                    "vehicle_id": sample_record.vehicle_id,
                    "odometer": sample_record.odometer,
                    "date": sample_record.date,
                    "notes": notes,
                });
                merge_json_objects(&mut expected, json!(log_type));

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }

    mod delete {
        use super::*;

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let sample_record = Faker.fake::<DeleteLogRecordResponse>();

                let expected = serde_json::Value::Null;

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }
}
