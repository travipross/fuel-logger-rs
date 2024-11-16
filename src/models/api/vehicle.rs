use std::ops::Deref;

use crate::{error::ApiError, models::db::Vehicle as DbVehicle, types::OdometerUnit};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

// Create
#[derive(Debug, Clone, serde::Deserialize, fake::Dummy, PartialEq)]
pub struct CreateVehicleBody {
    pub owner_id: Uuid,
    #[dummy(faker = "fake::faker::company::en::CompanyName()")]
    pub make: String,
    #[dummy(faker = "fake::faker::company::en::Buzzword()")]
    pub model: String,
    #[dummy(faker = "1950..2030")]
    pub year: u16,
    pub odometer_unit: Option<OdometerUnit>,
}

#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct CreateVehicleResponse {
    pub id: Uuid,
}

impl IntoResponse for CreateVehicleResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [("location", format!("/vehicles/{}", self.id))],
            Json(self),
        )
            .into_response()
    }
}

// Read
#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct ReadVehicleResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
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
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// List
#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct ListVehiclesResponse(Vec<ReadVehicleResponse>);

impl Deref for ListVehiclesResponse {
    type Target = Vec<ReadVehicleResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<ReadVehicleResponse> for ListVehiclesResponse {
    fn from_iter<T: IntoIterator<Item = ReadVehicleResponse>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoResponse for ListVehiclesResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

// Update
pub type UpdateVehicleBody = CreateVehicleBody;
pub type UpdateVehicleResponse = ReadVehicleResponse;

// Delete
#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct DeleteVehicleResponse;

impl IntoResponse for DeleteVehicleResponse {
    fn into_response(self) -> Response {
        (StatusCode::NO_CONTENT).into_response()
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use fake::{Fake, Faker};
    use serde_json::json;

    mod create {
        use super::*;
        mod request {
            use super::*;

            #[test]
            fn deserializes_with_odometer_unit() {
                // Arrange
                let mut expected = Faker.fake::<CreateVehicleBody>();
                expected.odometer_unit = Some(Faker.fake());

                let json = json!({
                    "owner_id": expected.owner_id,
                    "make": expected.make,
                    "model": expected.model,
                    "year": expected.year,
                    "odometer_unit": expected.odometer_unit,
                });

                // Act
                let deserialized = serde_json::from_value::<CreateVehicleBody>(json)
                    .expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }

            #[test]
            fn deserializes_with_no_odometer_unit() {
                // Arrange
                let mut expected = Faker.fake::<CreateVehicleBody>();
                expected.odometer_unit = None;

                let json = json!({
                    "owner_id": expected.owner_id,
                    "make": expected.make,
                    "model": expected.model,
                    "year": expected.year,
                    "odometer_unit": expected.odometer_unit,
                });

                // Act
                let deserialized = serde_json::from_value::<CreateVehicleBody>(json)
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
                let sample_record = Faker.fake::<CreateVehicleResponse>();
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
                let sample_records = Faker.fake::<Vec<ReadVehicleResponse>>();

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
            fn deserializes_with_odometer_unit() {
                // Arrange
                let make = Faker.fake::<String>();
                let model = Faker.fake::<String>();
                let year = Faker.fake::<u16>();
                let odometer_unit = Faker.fake::<Option<OdometerUnit>>();
                let owner_id = Faker.fake::<Uuid>();

                let json = json!({
                    "owner_id": owner_id,
                    "make": make,
                    "model": model,
                    "year": year,
                    "odometer_unit": odometer_unit,
                });

                let expected = UpdateVehicleBody {
                    owner_id,
                    make,
                    model,
                    year,
                    odometer_unit,
                };

                // Act
                let deserialized = serde_json::from_value::<UpdateVehicleBody>(json)
                    .expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }

            #[test]
            fn deserializes_with_no_odometer_unit() {
                // Arrange
                let make = Faker.fake::<String>();
                let model = Faker.fake::<String>();
                let year = Faker.fake::<u16>();
                let owner_id = Faker.fake::<Uuid>();

                let json = json!({
                    "owner_id": owner_id,
                    "make": make,
                    "model": model,
                    "year": year,
                });

                let expected = UpdateVehicleBody {
                    owner_id,
                    make,
                    model,
                    year,
                    odometer_unit: None,
                };

                // Act
                let deserialized = serde_json::from_value::<UpdateVehicleBody>(json)
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
                let sample_record = Faker.fake::<UpdateVehicleResponse>();

                let expected = json!({
                    "owner_id": sample_record.owner_id,
                    "make": sample_record.make,
                    "model": sample_record.model,
                    "year": sample_record.year,
                    "id": sample_record.id,
                    "odometer_unit": sample_record.odometer_unit,
                });

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
                let sample_record = Faker.fake::<DeleteVehicleResponse>();

                let expected = serde_json::Value::Null;

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }
}
