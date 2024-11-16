use std::ops::Deref;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::models::db::User as DbUser;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, fake::Dummy)]
pub struct CreateUserBody {
    #[dummy(faker = "fake::faker::name::en::FirstName()")]
    pub first_name: String,
    #[dummy(faker = "fake::faker::name::en::LastName()")]
    pub last_name: String,
    #[dummy(faker = "fake::faker::internet::en::Username()")]
    pub username: String,
    #[dummy(faker = "fake::faker::internet::en::FreeEmail()")]
    pub email: String,
}

#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct CreateUserResponse {
    pub id: Uuid,
}

impl IntoResponse for CreateUserResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [("location", format!("/users/{}", self.id).as_str())],
            Json(self),
        )
            .into_response()
    }
}

#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct ReadUserResponse {
    pub id: Uuid,
    #[dummy(faker = "fake::faker::name::en::FirstName()")]
    pub first_name: String,
    #[dummy(faker = "fake::faker::name::en::LastName()")]
    pub last_name: String,
    #[dummy(faker = "fake::faker::internet::en::Username()")]
    pub username: String,
    #[dummy(faker = "fake::faker::internet::en::FreeEmail()")]
    pub email: String,
}

impl IntoResponse for ReadUserResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl From<DbUser> for ReadUserResponse {
    fn from(value: DbUser) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            email: value.email,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct ListUsersResponse(Vec<ReadUserResponse>);

impl Deref for ListUsersResponse {
    type Target = Vec<ReadUserResponse>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl IntoResponse for ListUsersResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
impl FromIterator<ReadUserResponse> for ListUsersResponse {
    fn from_iter<T: IntoIterator<Item = ReadUserResponse>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

pub type UpdateUserBody = CreateUserBody;
pub type UpdateUserResponse = ReadUserResponse;

#[derive(Debug, Clone, serde::Serialize, fake::Dummy)]
pub struct DeleteUserResponse;

impl IntoResponse for DeleteUserResponse {
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
            fn deserializes_correctly() {
                // Arrange
                let expected = Faker.fake::<CreateUserBody>();

                let json = json!({
                    "first_name": expected.first_name,
                    "last_name": expected.last_name,
                    "username": expected.username,
                    "email": expected.email,
                });

                // Act
                let deserialized =
                    serde_json::from_value::<CreateUserBody>(json).expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }
        }

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let sample_record = Faker.fake::<CreateUserResponse>();
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
                let sample_record = Faker.fake::<ReadUserResponse>();

                let expected = json!({
                    "id": sample_record.id,
                    "first_name": sample_record.first_name,
                    "last_name": sample_record.last_name,
                    "username": sample_record.username,
                    "email": sample_record.email,
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
                let sample_records = Faker.fake::<Vec<ReadUserResponse>>();

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
            fn deserializes_correctly() {
                // Arrange
                let expected = Faker.fake::<UpdateUserBody>();

                let json = json!({
                    "first_name": expected.first_name,
                    "last_name": expected.last_name,
                    "username": expected.username,
                    "email": expected.email,
                });

                // Act
                let deserialized =
                    serde_json::from_value::<UpdateUserBody>(json).expect("could not deserialize");

                // Assert
                assert_eq!(deserialized, expected);
            }
        }

        mod response {
            use super::*;

            #[test]
            fn serializes_correctly() {
                // Arrange
                let sample_record = Faker.fake::<UpdateUserResponse>();

                let expected = json!({
                    "id": sample_record.id,
                    "first_name": sample_record.first_name,
                    "last_name": sample_record.last_name,
                    "username": sample_record.username,
                    "email": sample_record.email,
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
                let sample_record = Faker.fake::<DeleteUserResponse>();

                let expected = serde_json::Value::Null;

                // Act
                let serialized = serde_json::to_value(&sample_record).expect("could not serialize");

                // Assert
                assert_eq!(serialized, expected);
            }
        }
    }
}
