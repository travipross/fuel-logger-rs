pub mod api {
    use axum::{http::StatusCode, response::IntoResponse, Json};

    use super::db::User as DbUser;

    #[derive(Debug, serde::Deserialize, fake::Dummy)]
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

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct CreateUserResponse {
        pub id: uuid::Uuid,
    }

    impl IntoResponse for CreateUserResponse {
        fn into_response(self) -> axum::response::Response {
            (
                StatusCode::CREATED,
                [("location", format!("/users/{}", self.id).as_str())],
                Json(self),
            )
                .into_response()
        }
    }

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct ReadUserResponse {
        pub id: uuid::Uuid,
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
        fn into_response(self) -> axum::response::Response {
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

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct ListUsersResponse(Vec<ReadUserResponse>);

    impl IntoResponse for ListUsersResponse {
        fn into_response(self) -> axum::response::Response {
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

    #[derive(Debug, serde::Serialize, fake::Dummy)]
    pub struct DeleteUserResponse;

    impl IntoResponse for DeleteUserResponse {
        fn into_response(self) -> axum::response::Response {
            (StatusCode::NO_CONTENT).into_response()
        }
    }
}

pub mod db {
    use super::api::CreateUserBody as ApiCreateUserBody;

    #[derive(Debug, serde::Serialize, fake::Dummy, sqlx::FromRow)]
    pub struct User {
        pub id: uuid::Uuid,
        #[dummy(faker = "fake::faker::name::en::FirstName()")]
        pub first_name: String,
        #[dummy(faker = "fake::faker::name::en::LastName()")]
        pub last_name: String,
        #[dummy(faker = "fake::faker::internet::en::Username()")]
        pub username: String,
        #[dummy(faker = "fake::faker::internet::en::FreeEmail()")]
        pub email: String,
    }

    impl User {
        pub fn from_api_type(user_id: &uuid::Uuid, body: ApiCreateUserBody) -> Self {
            Self {
                id: *user_id,
                first_name: body.first_name,
                last_name: body.last_name,
                username: body.username,
                email: body.email,
            }
        }
    }
}
