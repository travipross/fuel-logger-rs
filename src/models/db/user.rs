use uuid::Uuid;

use crate::models::api::CreateUserBody as ApiCreateUserBody;

#[derive(Debug, Clone, PartialEq, fake::Dummy, sqlx::FromRow)]
pub struct User {
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

impl User {
    pub fn from_api_type(user_id: &Uuid, body: ApiCreateUserBody) -> Self {
        Self {
            id: *user_id,
            first_name: body.first_name,
            last_name: body.last_name,
            username: body.username,
            email: body.email,
        }
    }
}
