use chrono::{DateTime, Utc};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Serialize, PartialEq, Deserialize, Dummy, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "log_type")]
pub enum LogType {
    FuelUp {
        #[dummy(faker = "5.0..120.0")]
        fuel_amount: f32,
    },
    // TireRotation,
    // TireChange,
    // OilChange,
    // Repair,
    // WiperBladeReplacement,
    // BatteryReplacement,
    // Brakes
}

#[derive(Debug, Serialize, Dummy, PartialEq, Clone)]
pub struct LogRecord {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    #[serde(flatten)]
    pub log_type: LogType,
    #[dummy(faker = "0..500000")]
    pub odometer: u32,
}

#[derive(Debug, Deserialize, Dummy, PartialEq)]
pub struct LogRecordInput {
    pub date: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub log_type: LogType,
    #[dummy(faker = "0..500000")]
    pub odometer: u32,
}

#[cfg(test)]
mod log_record_tests {
    use fake::{Fake, Faker};
    use serde_json::json;

    use crate::models::{LogRecordInput, LogType};

    use super::{DateTime, LogRecord, Utc};

    #[test]
    fn log_record_serializes_fuel_up() {
        // Arrange
        let sample_record = LogRecord {
            log_type: LogType::FuelUp {
                fuel_amount: Faker.fake(),
            },
            ..Faker.fake()
        };

        let LogType::FuelUp { fuel_amount } = sample_record.log_type;

        #[allow(irrefutable_let_patterns)]
        let expected = json!({
            "id": sample_record.id,
            "date": format!("{:?}", sample_record.date),
            "log_type": "fuel_up",
            "fuel_amount": fuel_amount,
            "odometer": sample_record.odometer,
        });

        // Act
        let serialized = serde_json::to_value(&sample_record).unwrap();

        // Assert
        assert_eq!(serialized, expected);
    }

    #[test]
    fn log_record_input_deserializes_fuel_up_with_date() {
        // Arrange
        let date = Faker.fake::<Option<DateTime<Utc>>>();
        let fuel_amount = Faker.fake::<f32>();
        let odometer = Faker.fake::<u32>();

        let json = json!({
            "date": if let Some(date) = date {Some(format!("{:?}", date))} else {None},
            "log_type": "fuel_up",
            "fuel_amount": fuel_amount,
            "odometer": odometer,
        });

        let expected = LogRecordInput {
            log_type: LogType::FuelUp { fuel_amount },
            date: date,
            odometer,
        };

        // Act
        let deserialized = serde_json::from_value::<LogRecordInput>(json).unwrap();

        // Assert
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn log_record_input_deserializes_fuel_up_with_no_date() {
        // Arrange
        let fuel_amount = Faker.fake::<f32>();
        let odometer = Faker.fake::<u32>();

        let json = json!({
            "log_type": "fuel_up",
            "fuel_amount": fuel_amount,
            "odometer": odometer,
        });

        let expected = LogRecordInput {
            log_type: LogType::FuelUp { fuel_amount },
            date: None,
            odometer,
        };

        // Act
        let deserialized = serde_json::from_value::<LogRecordInput>(json).unwrap();

        // Assert
        assert_eq!(deserialized, expected);
    }
}
