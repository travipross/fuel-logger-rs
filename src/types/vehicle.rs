pub mod api {
    use crate::error::ApiError;
    use crate::types::primitives::OdometerUnit;
    use fake::faker::company::en::{Buzzword, CompanyName};
    use fake::Dummy;
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use super::db::Vehicle;

    #[derive(Debug, Deserialize, Dummy, PartialEq)]
    pub struct CreateVehicleBody {
        #[dummy(faker = "CompanyName()")]
        pub make: String,
        #[dummy(faker = "Buzzword()")]
        pub model: String,
        #[dummy(faker = "1950..2030")]
        pub year: u16,
        pub odometer_unit: Option<OdometerUnit>,
    }

    pub type UpdateVehicleBody = CreateVehicleBody;

    #[derive(Debug, Serialize, Dummy)]
    pub struct ReadVehicleResponse {
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

    #[derive(Debug, Serialize, Dummy)]
    pub struct CreateVehicleResponse {
        pub id: Uuid,
    }

    #[derive(Debug, Serialize, Dummy)]
    pub struct DeleteVehicleResponse;

    pub type UpdateVehicleResponse = ReadVehicleResponse;

    impl TryFrom<Vehicle> for ReadVehicleResponse {
        type Error = ApiError;
        fn try_from(value: Vehicle) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id,
                make: value.make,
                model: value.model,
                year: u16::try_from(value.year).map_err(|e| ApiError::Conversion(e.to_string()))?,
                owner_id: value.owner_id,
                odometer_unit: value.odometer_unit,
            })
        }
    }

    #[cfg(test)]
    mod api_type_tests {
        use super::*;
        use fake::{Fake, Faker};
        use serde_json::json;

        #[test]
        fn read_vehicle_response_serializes() {
            // Arrange
            let sample_record = Faker.fake::<ReadVehicleResponse>();

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

            let expected = CreateVehicleBody {
                make,
                model,
                year,
                odometer_unit,
            };

            // Act
            let deserialized = serde_json::from_value::<CreateVehicleBody>(json).unwrap();

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

            let expected = CreateVehicleBody {
                make,
                model,
                year,
                odometer_unit: None,
            };

            // Act
            let deserialized = serde_json::from_value::<CreateVehicleBody>(json).unwrap();

            // Assert
            assert_eq!(deserialized, expected);
        }
    }
}

pub mod db {
    use super::api::CreateVehicleBody as ApiCreateVehicleBody;
    use crate::types::primitives::OdometerUnit;
    use fake::{
        faker::company::en::{Buzzword, CompanyName},
        Dummy,
    };
    use serde::Deserialize;
    use sqlx::{postgres::PgRow, Row};
    use std::str::FromStr;
    use uuid::Uuid;

    #[derive(Debug, Deserialize, Dummy, PartialEq)]
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

    impl Vehicle {
        pub fn from_api_type(
            vehicle_id: &Uuid,
            owner_id: &Uuid,
            body: ApiCreateVehicleBody,
        ) -> Self {
            Self {
                id: *vehicle_id,
                make: body.make,
                model: body.model,
                year: body.year.into(),
                owner_id: *owner_id,
                odometer_unit: body.odometer_unit.unwrap_or_default(),
            }
        }
    }

    impl<'r> sqlx::FromRow<'r, PgRow> for Vehicle {
        fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
            let odometer_unit =
                OdometerUnit::from_str(row.try_get("odometer_unit")?).map_err(|e| {
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
}
