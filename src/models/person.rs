use fake::faker::internet::en::{FreeEmail, Username};
use fake::faker::name::en::{FirstName, LastName};
use fake::Dummy;
use serde::Serialize;

#[derive(Debug, Serialize, Dummy)]
pub struct FirstNameType(#[dummy(faker = "FirstName()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct LastNameType(#[dummy(faker = "LastName()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct UsernameType(#[dummy(faker = "Username()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct EmailAddressType(#[dummy(faker = "FreeEmail()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct Person {
    pub first_name: FirstNameType,
    pub last_name: LastNameType,
    pub username: UsernameType,
    pub email: EmailAddressType,
}
