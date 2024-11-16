use fake::Dummy;
use serde::Serialize;

#[derive(Debug, Serialize, Dummy)]
pub struct Person {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
}
