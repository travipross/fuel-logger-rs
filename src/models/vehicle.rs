use std::str::FromStr;

use fake::Dummy;
use serde::{Deserialize, Serialize};

use anyhow::anyhow;
use fake::faker::company::en::{Buzzword, CompanyName};
use sqlx::{postgres::PgRow, Postgres, Row};
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

impl<'q> sqlx::Encode<'q, Postgres> for OdometerUnit {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let repr = match self {
            Self::Imperial => "mi",
            Self::Metric => "km",
        };

        <&str as sqlx::Encode<Postgres>>::encode_by_ref(&repr, buf)
    }
}

impl sqlx::Decode<'_, Postgres> for OdometerUnit {
    fn decode(
        value: <Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let repr = <String as sqlx::Decode<Postgres>>::decode(value)?;

        match repr.as_str() {
            "mi" => Ok(Self::Imperial),
            "km" => Ok(Self::Metric),
            _ => Err("Unrecognized odometer unit type".into()),
        }
    }
}

impl sqlx::Type<Postgres> for OdometerUnit {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<Postgres>>::type_info()
    }
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

impl From<OdometerUnit> for &str {
    fn from(value: OdometerUnit) -> Self {
        match value {
            OdometerUnit::Metric => "km",
            OdometerUnit::Imperial => "mi",
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

#[derive(Debug, Deserialize, Dummy)]
pub struct CreateVehicle {
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

#[derive(Debug, Serialize, Dummy)]
pub struct CreateVehicleResponse {
    pub id: Uuid,
}

impl<'r> sqlx::FromRow<'r, PgRow> for Vehicle {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let odometer_unit = OdometerUnit::from_str(row.try_get("odometer_unit")?).map_err(|e| {
            sqlx::Error::ColumnDecode {
                index: "odometer_unit".to_owned(),
                source: e.into(),
            }
        })?;

        Ok(Self {
            id: row.try_get("id")?,
            make: row.try_get("make")?,
            model: row.try_get("model")?,
            owner_id: row.try_get("owner_id")?,
            year: row.try_get::<i32, _>("year")?.try_into().map_err(|e| {
                sqlx::Error::ColumnDecode {
                    index: "year".to_owned(),
                    source: Box::new(e),
                }
            })?,
            odometer_unit,
        })
    }
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

impl VehicleInput {
    pub fn into_db_input(self, owner_id: &Uuid) -> CreateVehicle {
        CreateVehicle {
            make: self.make,
            model: self.model,
            year: self.year,
            owner_id: *owner_id,
            odometer_unit: self.odometer_unit.unwrap_or_default(),
        }
    }
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
