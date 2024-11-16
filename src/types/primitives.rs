use std::str::FromStr;

use anyhow::anyhow;

use crate::error::ApiError;

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, sqlx::Type,
)]
#[serde(tag = "tire_rotation_type")]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
pub enum TireRotationType {
    FrontRear,
    Side,
    Diagonal,
}

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
pub enum TireType {
    Summer,
    Winter,
    AllSeason,
}

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
pub enum BrakeLocation {
    Front,
    Rear,
    All,
}

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
pub enum BrakeComponent {
    Rotors,
    Calipers,
    Both,
}

#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy, sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "fluid_type")]
#[sqlx(rename_all = "snake_case")]
#[sqlx(type_name = "text")]
pub enum FluidType {
    Wiper,
    Transmission,
    Brake,
    Coolant,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize, fake::Dummy)]
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

#[cfg(test)]
mod serde_tests {
    use super::*;
    use serde_json::json;

    mod tire_rotation_type {
        use super::*;

        #[test_case::test_case(TireRotationType::FrontRear => json!({"tire_rotation_type": "front_rear"}))]
        #[test_case::test_case(TireRotationType::Diagonal => json!({"tire_rotation_type": "diagonal"}))]
        #[test_case::test_case(TireRotationType::Side => json!({"tire_rotation_type": "side"}))]
        fn serializes_correctly(rotation_type: TireRotationType) -> serde_json::Value {
            // Act
            serde_json::to_value(rotation_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!({"tire_rotation_type": "front_rear"}) => TireRotationType::FrontRear)]
        #[test_case::test_case(json!({"tire_rotation_type": "diagonal"}) => TireRotationType::Diagonal)]
        #[test_case::test_case(json!({"tire_rotation_type": "side"}) => TireRotationType::Side)]
        fn deserializes_correctly(value: serde_json::Value) -> TireRotationType {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }

    mod tire_type {
        use super::*;

        #[test_case::test_case(TireType::Winter => json!("winter"))]
        #[test_case::test_case(TireType::Summer => json!("summer"))]
        #[test_case::test_case(TireType::AllSeason => json!("all_season"))]
        fn serializes_correctly(tire_type: TireType) -> serde_json::Value {
            serde_json::to_value(tire_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!("winter") => TireType::Winter)]
        #[test_case::test_case(json!("summer") => TireType::Summer)]
        #[test_case::test_case(json!("all_season") => TireType::AllSeason)]
        fn deserializes_correctly(value: serde_json::Value) -> TireType {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }

    mod fluid_type {
        use super::*;

        #[test_case::test_case(FluidType::Wiper => json!({"fluid_type": "wiper"}))]
        #[test_case::test_case(FluidType::Transmission => json!({"fluid_type": "transmission"}))]
        #[test_case::test_case(FluidType::Coolant => json!({"fluid_type": "coolant"}))]
        #[test_case::test_case(FluidType::Brake => json!({"fluid_type": "brake"}))]
        fn serializes_correctly(tire_type: FluidType) -> serde_json::Value {
            serde_json::to_value(tire_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!({"fluid_type": "wiper"}) => FluidType::Wiper)]
        #[test_case::test_case(json!({"fluid_type": "transmission"}) => FluidType::Transmission)]
        #[test_case::test_case(json!({"fluid_type": "brake"}) => FluidType::Brake)]
        #[test_case::test_case(json!({"fluid_type": "coolant"}) => FluidType::Coolant)]
        fn deserializes_correctly(value: serde_json::Value) -> FluidType {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }

    mod brake_component {
        use super::*;

        #[test_case::test_case(BrakeComponent::Calipers => json!("calipers"))]
        #[test_case::test_case(BrakeComponent::Rotors => json!("rotors"))]
        #[test_case::test_case(BrakeComponent::Both => json!("both"))]
        fn serializes_correctly(tire_type: BrakeComponent) -> serde_json::Value {
            serde_json::to_value(tire_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!("calipers") => BrakeComponent::Calipers)]
        #[test_case::test_case(json!("rotors") => BrakeComponent::Rotors)]
        #[test_case::test_case(json!("both") => BrakeComponent::Both)]
        fn deserializes_correctly(value: serde_json::Value) -> BrakeComponent {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }

    mod brake_location {
        use super::*;

        #[test_case::test_case(BrakeLocation::Front => json!("front"))]
        #[test_case::test_case(BrakeLocation::Rear => json!("rear"))]
        #[test_case::test_case(BrakeLocation::All => json!("all"))]
        fn serializes_correctly(tire_type: BrakeLocation) -> serde_json::Value {
            serde_json::to_value(tire_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!("front") => BrakeLocation::Front)]
        #[test_case::test_case(json!("rear") => BrakeLocation::Rear)]
        #[test_case::test_case(json!("all") => BrakeLocation::All)]
        fn deserializes_correctly(value: serde_json::Value) -> BrakeLocation {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }

    mod odometer_unit {
        use super::*;

        #[test_case::test_case(OdometerUnit::Metric => json!("km"))]
        #[test_case::test_case(OdometerUnit::Imperial => json!("mi"))]
        fn serializes_correctly(tire_type: OdometerUnit) -> serde_json::Value {
            serde_json::to_value(tire_type).expect("could not serialize value")
        }

        #[test_case::test_case(json!("km") => OdometerUnit::Metric)]
        #[test_case::test_case(json!("mi") => OdometerUnit::Imperial)]
        fn deserializes_correctly(value: serde_json::Value) -> OdometerUnit {
            serde_json::from_value(value).expect("could not deserialize type")
        }
    }
}
