use crate::types::primitives::{
    BrakeComponent, BrakeLocation, FluidType, TireRotationType, TireType,
};

pub mod api {
    use super::{db::LogRecord as DbLogRecord, LogType};
    use crate::error::ApiError;
    use axum::{http::StatusCode, response::IntoResponse, Json};

    // Create
    #[derive(Debug, PartialEq, serde::Deserialize, fake::Dummy)]
    pub struct CreateLogRecordBody {
        pub date: Option<chrono::DateTime<chrono::Utc>>,
        pub vehicle_id: uuid::Uuid,
        #[serde(flatten)]
        pub log_type: LogType,
        #[dummy(faker = "0..500000")]
        pub odometer: u16,
        pub notes: Option<String>,
    }

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct CreateLogRecordResponse {
        pub id: uuid::Uuid,
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
        pub id: uuid::Uuid,
        pub vehicle_id: uuid::Uuid,
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

    //         assert_eq!(deserialized, expected);
    //     }
    // }
}

pub mod db {
    use super::api::CreateLogRecordBody as ApiCreateLogRecordBody;
    use super::LogType;
    use sqlx::{postgres::PgRow, FromRow, Row};

    #[derive(Debug, fake::Dummy, PartialEq, Clone, sqlx::FromRow)]
    pub struct LogRecord {
        pub id: uuid::Uuid,
        pub vehicle_id: uuid::Uuid,
        pub date: chrono::DateTime<chrono::Utc>,
        #[sqlx(flatten)]
        pub log_type: LogType,
        pub odometer: i32,
        pub notes: Option<String>,
    }

    impl LogRecord {
        pub fn from_api_type(log_record_id: &uuid::Uuid, body: ApiCreateLogRecordBody) -> Self {
            Self {
                id: *log_record_id,
                vehicle_id: body.vehicle_id,
                date: body.date.unwrap_or_else(chrono::Utc::now),
                log_type: body.log_type,
                odometer: body.odometer.into(),
                notes: body.notes,
            }
        }
    }

    impl<'r> FromRow<'r, PgRow> for LogRecord {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
            let id: uuid::Uuid = row.try_get("id")?;
            let date: chrono::DateTime<chrono::Utc> = row.try_get("log_date")?;
            let vehicle_id: uuid::Uuid = row.try_get("vehicle_id")?;
            let odometer: u16 = row.try_get::<i32, _>("odometer")?.try_into().map_err(|e| {
                sqlx::Error::ColumnDecode {
                    index: "odometer".to_owned(),
                    source: Box::new(e),
                }
            })?;
            let notes = row.try_get::<String, _>("notes").ok();
            let log_type_name: &str = row.try_get("log_type")?;
            let log_type_enum = match log_type_name {
                "fuel_up" => {
                    let fuel_amount = row.try_get("fuel_amount")?;
                    LogType::FuelUp { fuel_amount }
                }
                "tire_rotation" => {
                    let rotation_type = row.try_get("tire_rotation_type")?;
                    LogType::TireRotation(rotation_type)
                }
                "tire_change" => LogType::TireChange {
                    rotation: row.try_get("tire_rotation_type")?,
                    tire_type: row.try_get("tire_type")?,
                    new: row.try_get("new_tires")?,
                },
                "oil_change" => LogType::OilChange,
                "repair" => LogType::Repair,
                "wiper_blade_replacement" => LogType::WiperBladeReplacement,
                "battery_replacement" => LogType::BatteryReplacement,
                "brake_replacement" => LogType::BrakeReplacement {
                    location: row.try_get("brake_location")?,
                    component: row.try_get("brake_part")?,
                },
                "fluids" => LogType::Fluids(row.try_get("fluid_type")?),
                _ => {
                    return Err(sqlx::Error::Decode(
                        format!("unrecognized log_type: {log_type_name}").into(),
                    ))
                }
            };

            Ok(Self {
                id,
                vehicle_id,
                date,
                odometer: odometer.into(),
                notes,
                log_type: log_type_enum,
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, fake::Dummy)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "log_type")]
pub enum LogType {
    FuelUp {
        #[dummy(faker = "5.0..120.0")]
        fuel_amount: f32,
    },
    TireRotation(TireRotationType),
    TireChange {
        #[serde(flatten)]
        rotation: Option<TireRotationType>,
        tire_type: TireType,
        new: bool,
    },
    OilChange,
    Repair,
    WiperBladeReplacement,
    BatteryReplacement,
    BrakeReplacement {
        location: BrakeLocation,
        #[serde(rename = "brake_part")]
        component: BrakeComponent,
    },
    Fluids(FluidType),
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FuelUp { .. } => write!(f, "fuel_up"),
            Self::TireRotation(_) => write!(f, "tire_rotation"),
            Self::TireChange { .. } => write!(f, "tire_change"),
            Self::OilChange => write!(f, "oil_change"),
            Self::Repair => write!(f, "repair"),
            Self::WiperBladeReplacement => write!(f, "wiper_blade_replacement"),
            Self::BatteryReplacement => write!(f, "battery_replacement"),
            Self::BrakeReplacement { .. } => write!(f, "brake_replacement"),
            Self::Fluids(_) => write!(f, "fluids"),
        }
    }
}
