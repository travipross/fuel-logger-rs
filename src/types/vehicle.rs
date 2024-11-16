pub mod api {
    use super::db::Vehicle as DbVehicle;
    use crate::error::ApiError;
    use crate::types::primitives::OdometerUnit;
    use axum::{http::StatusCode, response::IntoResponse, Json};

    // Create
    #[derive(Debug, serde::Deserialize, fake::Dummy, PartialEq)]
    pub struct CreateVehicleBody {
        pub owner_id: uuid::Uuid,
        #[dummy(faker = "fake::faker::company::en::CompanyName()")]
        pub make: String,
        #[dummy(faker = "fake::faker::company::en::Buzzword()")]
        pub model: String,
        #[dummy(faker = "1950..2030")]
        pub year: u16,
        pub odometer_unit: Option<OdometerUnit>,
    }

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct CreateVehicleResponse {
        pub id: uuid::Uuid,
    }

    impl IntoResponse for CreateVehicleResponse {
        fn into_response(self) -> axum::response::Response {
            (
                StatusCode::CREATED,
                [("location", format!("/vehicles/{}", self.id))],
                Json(self),
            )
                .into_response()
        }
    }

    // Read
    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct ReadVehicleResponse {
        pub id: uuid::Uuid,
        pub owner_id: uuid::Uuid,
        #[dummy(faker = "fake::faker::company::en::CompanyName()")]
        pub make: String,
        #[dummy(faker = "fake::faker::company::en::Buzzword()")]
        pub model: String,
        #[dummy(faker = "1950..2030")]
        pub year: u16,
        // TODO: Add owner_id
        pub odometer_unit: OdometerUnit,
    }

    impl TryFrom<DbVehicle> for ReadVehicleResponse {
        type Error = ApiError;
        fn try_from(value: DbVehicle) -> Result<Self, Self::Error> {
            Ok(Self {
                id: value.id,
                owner_id: value.owner_id,
                make: value.make,
                model: value.model,
                year: u16::try_from(value.year).map_err(|e| ApiError::Conversion(e.to_string()))?,
                odometer_unit: value.odometer_unit,
            })
        }
    }

    impl IntoResponse for ReadVehicleResponse {
        fn into_response(self) -> axum::response::Response {
            (StatusCode::OK, Json(self)).into_response()
        }
    }

    // List
    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct ListVehiclesResponse(Vec<ReadVehicleResponse>);

    impl FromIterator<ReadVehicleResponse> for ListVehiclesResponse {
        fn from_iter<T: IntoIterator<Item = ReadVehicleResponse>>(iter: T) -> Self {
            Self(iter.into_iter().collect())
        }
    }

    impl IntoResponse for ListVehiclesResponse {
        fn into_response(self) -> axum::response::Response {
            (StatusCode::OK, Json(self)).into_response()
        }
    }

    // Update
    pub type UpdateVehicleBody = CreateVehicleBody;
    pub type UpdateVehicleResponse = ReadVehicleResponse;

    // Delete
    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct DeleteVehicleResponse;

    impl IntoResponse for DeleteVehicleResponse {
        fn into_response(self) -> axum::response::Response {
            (StatusCode::NO_CONTENT).into_response()
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
                "owner_id": sample_record.owner_id,
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
            let owner_id = Faker.fake::<uuid::Uuid>();

            let json = json!({
                "owner_id": owner_id,
                "make": make,
                "model": model,
                "year": year,
                "odometer_unit": odometer_unit,
            });

            let expected = CreateVehicleBody {
                owner_id,
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
            let owner_id = Faker.fake::<uuid::Uuid>();

            let json = json!({
                "owner_id": owner_id,
                "make": make,
                "model": model,
                "year": year,
            });

            let expected = CreateVehicleBody {
                owner_id,
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

    #[derive(Debug, PartialEq, serde::Deserialize, fake::Dummy, sqlx::FromRow)]
    pub struct Vehicle {
        pub id: uuid::Uuid,
        pub owner_id: uuid::Uuid,
        #[dummy(faker = "fake::faker::company::en::CompanyName()")]
        pub make: String,
        #[dummy(faker = "fake::faker::company::en::Buzzword()")]
        pub model: String,
        #[dummy(faker = "1950..2030")]
        pub year: i32,
        #[serde(skip)]
        pub odometer_unit: OdometerUnit,
    }

    impl Vehicle {
        pub fn from_api_type(vehicle_id: &uuid::Uuid, body: ApiCreateVehicleBody) -> Self {
            Self {
                id: *vehicle_id,
                owner_id: body.owner_id,
                make: body.make,
                model: body.model,
                year: body.year.into(),
                odometer_unit: body.odometer_unit.unwrap_or_default(),
            }
        }
    }
}
