use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::name::en::{FirstName, LastName};
use fake::Dummy;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Dummy)]
pub struct User {
    pub id: Uuid,
    #[dummy(faker = "FirstName()")]
    pub first_name: String,
    #[dummy(faker = "LastName()")]
    pub last_name: String,
    #[dummy(faker = "Username()")]
    pub username: String,
    #[dummy(faker = "FreeEmail()")]
    pub email: String,
}
