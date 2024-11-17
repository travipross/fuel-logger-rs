use crate::{
    error::ApiError,
    models::api::CreateLogRecordBody as ApiCreateLogRecordBody,
    types::{BrakeComponent, BrakeLocation, FluidType, LogType, TireRotationType, TireType},
};
use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

// TODO: Convert this to better reflect table structure with Options instead of LogType struct
#[derive(Debug, Clone, fake::Dummy, sqlx::FromRow)]
pub struct LogRecord {
    pub id: Uuid,
    pub vehicle_id: Uuid,
    pub date: DateTime<Utc>,
    #[sqlx(flatten)]
    pub log_type: LogType,
    #[dummy(faker = "100..1000000")]
    pub odometer: i32,
    pub notes: Option<String>,
}

impl PartialEq for LogRecord {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.vehicle_id == other.vehicle_id
            && self.date.timestamp_millis() == other.date.timestamp_millis()
            && self.log_type == other.log_type
            && self.odometer == other.odometer
            && self.notes == other.notes
    }
}

impl LogRecord {
    pub fn from_api_type(
        log_record_id: &Uuid,
        body: ApiCreateLogRecordBody,
    ) -> Result<Self, ApiError> {
        Ok(Self {
            id: *log_record_id,
            vehicle_id: body.vehicle_id,
            date: body.date.unwrap_or_else(Utc::now),
            log_type: body.log_type,
            odometer: body.odometer.try_into().map_err(|_| {
                ApiError::Conversion("could not convert odometer reading into i32".to_owned())
            })?,
            notes: body.notes,
        })
    }

    pub fn log_type(&self) -> String {
        match self.log_type {
            LogType::FuelUp { .. } => "fuel_up",
            LogType::TireRotation(_) => "tire_rotation",
            LogType::TireChange { .. } => "tire_change",
            LogType::Fluids(_) => "fluids",
            LogType::WiperBladeReplacement => "wiper_blade_replacement",
            LogType::Repair => "repair",
            LogType::BrakeReplacement { .. } => "brake_replacement",
            LogType::BatteryReplacement => "battery_replacement",
            LogType::OilChange => "oil_change",
        }
        .to_owned()
    }

    pub fn fuel_amount(&self) -> Option<f32> {
        if let LogType::FuelUp { fuel_amount } = self.log_type {
            Some(fuel_amount)
        } else {
            None
        }
    }

    pub fn tire_rotation_type(&self) -> Option<TireRotationType> {
        match self.log_type.clone() {
            LogType::TireRotation(tire_rotation_type) => Some(tire_rotation_type),
            LogType::TireChange { rotation, .. } => rotation,
            _ => None,
        }
    }

    pub fn tire_type(&self) -> Option<TireType> {
        if let LogType::TireChange { tire_type, .. } = self.log_type.clone() {
            Some(tire_type)
        } else {
            None
        }
    }

    pub fn new_tires(&self) -> Option<bool> {
        if let LogType::TireChange { new, .. } = self.log_type {
            Some(new)
        } else {
            None
        }
    }

    pub fn brake_location(&self) -> Option<BrakeLocation> {
        if let LogType::BrakeReplacement { location, .. } = self.log_type.clone() {
            Some(location)
        } else {
            None
        }
    }

    pub fn brake_part(&self) -> Option<BrakeComponent> {
        if let LogType::BrakeReplacement { component, .. } = self.log_type.clone() {
            Some(component)
        } else {
            None
        }
    }

    pub fn fluid_type(&self) -> Option<FluidType> {
        if let LogType::Fluids(fluid_type) = self.log_type.clone() {
            Some(fluid_type)
        } else {
            None
        }
    }
}

impl<'r> FromRow<'r, PgRow> for LogRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: Uuid = row.try_get("id")?;
        let date: DateTime<Utc> = row.try_get("log_date")?;
        let vehicle_id: Uuid = row.try_get("vehicle_id")?;
        let odometer = row.try_get::<i32, _>("odometer")?;
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
            odometer,
            notes,
            log_type: log_type_enum,
        })
    }
}
