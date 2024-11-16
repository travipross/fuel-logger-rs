use crate::{models::api::CreateLogRecordBody as ApiCreateLogRecordBody, types::LogType};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Debug, fake::Dummy, PartialEq, Clone, sqlx::FromRow)]
pub struct LogRecord {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub date: chrono::DateTime<chrono::Utc>,
    #[sqlx(flatten)]
    pub log_type: LogType,
    pub odometer: i32,
    pub notes: Option<String>,
}

impl LogRecord {
    pub fn from_api_type(log_record_id: &Uuid, body: ApiCreateLogRecordBody) -> Self {
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
        let id: Uuid = row.try_get("id")?;
        let date: chrono::DateTime<chrono::Utc> = row.try_get("log_date")?;
        let vehicle_id: Uuid = row.try_get("vehicle_id")?;
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
