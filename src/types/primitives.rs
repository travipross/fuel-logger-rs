use std::str::FromStr;

use anyhow::anyhow;
use fake::Dummy;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq)]
#[serde(tag = "rotation_type")]
#[serde(rename_all = "snake_case")]
pub enum TireRotationType {
    FrontRear,
    Side,
    Diagonal,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TireType {
    Summer,
    Winter,
    AllSeason,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrakeLocation {
    Front,
    Rear,
    All,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BrakeComponent {
    Rotors,
    Calipers,
    Both,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FluidType {
    Wiper,
    Transmission,
    Brake,
    Coolant,
}

#[derive(Debug, Serialize, Deserialize, Dummy, Default, PartialEq)]
pub enum OdometerUnit {
    #[serde(rename = "km")]
    #[default]
    Metric,
    #[serde(rename = "mi")]
    Imperial,
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for OdometerUnit {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let repr = match self {
            Self::Imperial => "mi",
            Self::Metric => "km",
        };

        <&str as sqlx::Encode<sqlx::Postgres>>::encode_by_ref(&repr, buf)
    }
}

impl sqlx::Decode<'_, sqlx::Postgres> for OdometerUnit {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'_>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let repr = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;

        match repr.as_str() {
            "mi" => Ok(Self::Imperial),
            "km" => Ok(Self::Metric),
            _ => Err("Unrecognized odometer unit type".into()), // TODO: Error
        }
    }
}

impl sqlx::Type<sqlx::Postgres> for OdometerUnit {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl FromStr for OdometerUnit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "km" => Ok(Self::Metric),
            "mi" => Ok(Self::Imperial),
            _ => Err(anyhow!("Invalid type")), // TODO: Error
        }
    }
}

impl From<OdometerUnit> for &str {
    fn from(value: OdometerUnit) -> Self {
        match value {
            OdometerUnit::Metric => "km",
            OdometerUnit::Imperial => "mi",
        }
    }
}

impl TryFrom<String> for OdometerUnit {
    type Error = ApiError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "km" => Ok(Self::Metric),
            "mi" => Ok(Self::Imperial),
            _ => Err(ApiError::Conversion(format!(
                "unrecognized odometer unit: {value}. Must be \"km\" or \"mi\""
            ))),
        }
    }
}

impl ToString for OdometerUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Metric => "km".to_owned(),
            Self::Imperial => "mi".to_owned(),
        }
    }
}
