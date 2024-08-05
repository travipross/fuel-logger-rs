use std::{error::Error, str::FromStr};

use fake::Dummy;
use serde::{de, Deserialize, Serialize};

use anyhow::anyhow;
use fake::faker::company::en::{Buzzword, CompanyName};
use sqlx::{postgres::PgArguments, Decode, Encode, Postgres};
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

impl FromStr for OdometerUnit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "km" => Ok(Self::Metric),
            "mi" => Ok(Self::Imperial),
            _ => Err(anyhow!("Invalid type")),
        }
    }
}

impl ToString for OdometerUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Metric => "km".to_owned(),
            Self::Imperial => "mi".to_owned(),
        }
    }
}

impl<'q> Encode<'q, Postgres> for OdometerUnit {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let val = self.to_string().as_str();
        <&str as Encode<Postgres>>::encode(val, buf)
    }
}

impl<'r> Decode<'r, Postgres> for OdometerUnit {
    fn decode(
        value: <Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as Decode<Postgres>>::decode(value)?;
        OdometerUnit::from_str(s).map_err(|err| Box::new(err) as _)
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy, sqlx::Decode)]
pub struct Vehicle {
    pub id: Uuid,
    #[dummy(faker = "CompanyName()")]
    pub make: String,
    #[dummy(faker = "Buzzword()")]
    pub model: String,
    #[dummy(faker = "1950..2030")]
    pub year: i32,
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
