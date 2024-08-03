use fake::Dummy;
use serde::{Deserialize, Serialize};

use fake::faker::company::en::{Buzzword, CompanyName};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Dummy, Default, PartialEq)]
pub enum OdometerUnit {
    #[serde(rename = "km")]
    #[default]
    Metric,
    #[serde(rename = "mi")]
    Imperial,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub struct Vehicle {
    pub id: Uuid,
    #[dummy(faker = "CompanyName()")]
    pub make: String,
    #[dummy(faker = "Buzzword()")]
    pub model: String,
    #[dummy(faker = "1950..2030")]
    pub year: u16,
    #[serde(skip)]
    pub owner_id: Uuid,
    pub odometer_unit: OdometerUnit,
}

#[derive(Debug, Deserialize, Dummy, PartialEq)]
pub struct VehicleInput {
    #[dummy(faker = "CompanyName()")]
    pub make: String,
    #[dummy(faker = "Buzzword()")]
    pub model: String,
    #[dummy(faker = "1950..2030")]
    pub year: u16,
    pub odometer_unit: Option<OdometerUnit>,
}

#[cfg(test)]
mod vehicle_tests {
    use fake::{Fake, Faker};
    use serde_json::json;

    use crate::models::{OdometerUnit, Vehicle, VehicleInput};

    #[test]
    fn vehicle_serializes() {
        // Arrange
        let sample_record = Faker.fake::<Vehicle>();

        let expected = json!({
            "make": sample_record.make,
            "model": sample_record.model,
            "year": sample_record.year,
            "id": sample_record.id,
            "odometer_unit": sample_record.odometer_unit,
        });

        // Act
        let serialized = serde_json::to_value(&sample_record).unwrap();

        // Assert
        assert_eq!(serialized, expected);
    }

    #[test]
    fn vehicle_input_deserializes_with_odometer_unit() {
        // Arrange
        let make = Faker.fake::<String>();
        let model = Faker.fake::<String>();
        let year = Faker.fake::<u16>();
        let odometer_unit = Faker.fake::<Option<OdometerUnit>>();

        let json = json!({
            "make": make,
            "model": model,
            "year": year,
            "odometer_unit": odometer_unit,
        });

        let expected = VehicleInput {
            make,
            model,
            year,
            odometer_unit,
        };

        // Act
        let deserialized = serde_json::from_value::<VehicleInput>(json).unwrap();

        // Assert
        assert_eq!(deserialized, expected);
    }

    #[test]
    fn vehicle_input_deserializes_with_no_odometer_unit() {
        // Arrange
        let make = Faker.fake::<String>();
        let model = Faker.fake::<String>();
        let year = Faker.fake::<u16>();

        let json = json!({
            "make": make,
            "model": model,
            "year": year,
        });

        let expected = VehicleInput {
            make,
            model,
            year,
            odometer_unit: None,
        };

        // Act
        let deserialized = serde_json::from_value::<VehicleInput>(json).unwrap();

        // Assert
        assert_eq!(deserialized, expected);
    }
}
