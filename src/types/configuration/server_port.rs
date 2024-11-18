use std::{fmt::Display, ops::Deref};

use fake::Faker;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ServerPort(u16);

impl Display for ServerPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ServerPort {
    fn default() -> Self {
        Self(3000)
    }
}

impl fake::Dummy<Faker> for ServerPort {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &Faker, rng: &mut R) -> Self {
        Self(rng.gen_range(1024..=49151))
    }
}

impl Deref for ServerPort {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<u16> for ServerPort {
    fn eq(&self, other: &u16) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ServerPort> for u16 {
    fn eq(&self, other: &ServerPort) -> bool {
        *self == other.0
    }
}

impl From<u16> for ServerPort {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<ServerPort> for u16 {
    fn from(value: ServerPort) -> Self {
        value.0
    }
}
