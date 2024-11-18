use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy)]
#[serde(transparent)]
pub struct ServerHost(#[dummy(faker = "fake::faker::internet::en::IPv4()")] String);

impl Default for ServerHost {
    fn default() -> Self {
        Self("0.0.0.0".to_owned())
    }
}

impl Display for ServerHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Deref for ServerHost {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<String> for ServerHost {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ServerHost> for String {
    fn eq(&self, other: &ServerHost) -> bool {
        *self == other.0
    }
}

impl From<String> for ServerHost {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<ServerHost> for String {
    fn from(value: ServerHost) -> Self {
        value.0
    }
}
