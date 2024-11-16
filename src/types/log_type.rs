use crate::types::{BrakeComponent, BrakeLocation, FluidType, TireRotationType, TireType};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, fake::Dummy)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "log_type")]
pub enum LogType {
    FuelUp {
        #[dummy(faker = "5.0..120.0")]
        fuel_amount: f32,
    },
    TireChange {
        #[serde(flatten)]
        rotation: Option<TireRotationType>,
        tire_type: TireType,
        new: bool,
    },
    BrakeReplacement {
        #[serde(rename = "brake_location")]
        location: BrakeLocation,
        #[serde(rename = "brake_part")]
        component: BrakeComponent,
    },
    TireRotation(TireRotationType),
    Fluids(FluidType),
    OilChange,
    Repair,
    WiperBladeReplacement,
    BatteryReplacement,
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FuelUp { .. } => write!(f, "fuel_up"),
            Self::BrakeReplacement { .. } => write!(f, "brake_replacement"),
            Self::TireChange { .. } => write!(f, "tire_change"),
            Self::TireRotation(_) => write!(f, "tire_rotation"),
            Self::Fluids(_) => write!(f, "fluids"),
            Self::OilChange => write!(f, "oil_change"),
            Self::Repair => write!(f, "repair"),
            Self::WiperBladeReplacement => write!(f, "wiper_blade_replacement"),
            Self::BatteryReplacement => write!(f, "battery_replacement"),
        }
    }
}

#[cfg(test)]
mod serde_tests {
    use super::*;
    use fake::{Fake, Faker};
    use serde_json::json;

    mod fuel_up {
        use serde_json::json;

        use super::*;

        #[test]
        fn serializes_correctly() {
            // Arrange
            let fuel_amount = Faker.fake::<f32>();
            let sample_log_type = LogType::FuelUp { fuel_amount };

            let expected = json!({
                "fuel_amount": fuel_amount,
                "log_type": "fuel_up"
            });

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }

        #[test]
        fn deserializes_correctly() {
            // Arrange
            let fuel_amount = Faker.fake::<f32>();
            let sample_log_type = json!({"log_type": "fuel_up", "fuel_amount": fuel_amount});
            let expected = LogType::FuelUp { fuel_amount };

            // Act
            let deserialized = serde_json::from_value::<LogType>(sample_log_type)
                .expect("could not deserialize value");

            // Assert
            assert_eq!(deserialized, expected);
        }
    }

    mod tire_rotation {
        use super::*;
        use crate::utils::test_utils::merge_json_objects;

        #[test]
        fn serializes_correctly() {
            // Arrange
            let inner = Faker.fake::<TireRotationType>();
            let sample_log_type = LogType::TireRotation(inner.clone());

            let mut expected = json!({
                "log_type": "tire_rotation",
            });

            merge_json_objects(&mut expected, json!(inner));

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }

        #[test]
        fn deserializes_correctly() {
            // Arrange
            let rotation_type = Faker.fake::<TireRotationType>();
            let mut sample_log_type = json!({"log_type": "tire_rotation"});

            merge_json_objects(&mut sample_log_type, json!(rotation_type));
            let expected = LogType::TireRotation(rotation_type);

            // Act
            let deserialized = serde_json::from_value::<LogType>(sample_log_type)
                .expect("could not deserialize value");

            // Assert
            assert_eq!(deserialized, expected);
        }
    }

    mod tire_change {
        use super::*;
        use crate::utils::test_utils::merge_json_objects;

        #[test]
        fn serializes_correctly_with_tire_rotation() {
            // Arrange
            let rotation = Faker.fake();
            let tire_type = Faker.fake();
            let new = Faker.fake();

            let mut expected = json!({
                "tire_type": &tire_type,
                "new": &new,
                "log_type": "tire_change"
            });

            merge_json_objects(&mut expected, json!(rotation));

            let sample_log_type = LogType::TireChange {
                rotation: Some(rotation),
                tire_type,
                new,
            };

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }

        #[test]
        fn serializes_correctly_without_tire_rotation() {
            // Arrange
            let tire_type = Faker.fake();
            let new = Faker.fake();

            let expected = json!({
                "tire_type": &tire_type,
                "new": &new,
                "log_type": "tire_change"
            });

            let sample_log_type = LogType::TireChange {
                rotation: None,
                tire_type,
                new,
            };

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }
    }

    mod brake_replacement {
        use super::*;

        #[test]
        fn serializes_correctly() {
            // Arrange
            let location = Faker.fake();
            let component = Faker.fake();

            let expected = json!({
                "log_type": "brake_replacement",
                "brake_part": component,
                "brake_location": location,
            });

            let sample_log_type = LogType::BrakeReplacement {
                location,
                component,
            };

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }
    }

    mod fluids {
        use crate::utils::test_utils::merge_json_objects;

        use super::*;

        #[test]
        fn serializes_correctly() {
            // Arrange
            let fluid_type = Faker.fake();

            let mut expected = json!({
                "log_type": "fluids",
            });
            merge_json_objects(&mut expected, json!(fluid_type));

            let sample_log_type = LogType::Fluids(fluid_type);

            // Act
            let serialized =
                serde_json::to_value(sample_log_type).expect("could not serialize value");

            // Assert
            assert_eq!(serialized, expected);
        }
    }

    #[test_case::test_case(LogType::OilChange => json!({"log_type": "oil_change"}) ; "oil_change")]
    #[test_case::test_case(LogType::BatteryReplacement => json!({"log_type": "battery_replacement"}) ; "battery_replacement")]
    #[test_case::test_case(LogType::Repair => json!({"log_type": "repair"}) ; "repair")]
    #[test_case::test_case(LogType::WiperBladeReplacement => json!({"log_type": "wiper_blade_replacement"}) ; "wiper_blade_replacement")]
    fn unit_types_serialize_correctly(log_type: LogType) -> serde_json::Value {
        // Act
        serde_json::to_value(log_type).expect("could not serialize value")
    }

    #[test_case::test_case(json!({"log_type": "oil_change"}) => LogType::OilChange ; "oil_change")]
    #[test_case::test_case(json!({"log_type": "battery_replacement"}) => LogType::BatteryReplacement ; "battery_replacement")]
    #[test_case::test_case(json!({"log_type": "repair"}) => LogType::Repair ; "repair")]
    #[test_case::test_case(json!({"log_type": "wiper_blade_replacement"}) => LogType::WiperBladeReplacement ; "wiper_blade_replacement")]
    fn unit_types_deserialize_correctly(value: serde_json::Value) -> LogType {
        // Act
        serde_json::from_value(value).expect("could not deserialize")
    }

    #[test_case::test_case(json!({"log_type": "oil_change", "extra_field": Faker.fake::<String>()}) => LogType::OilChange ; "oil_change")]
    #[test_case::test_case(json!({"log_type": "battery_replacement", "extra_field": Faker.fake::<String>()}) => LogType::BatteryReplacement ; "battery_replacement")]
    #[test_case::test_case(json!({"log_type": "repair", "extra_field": Faker.fake::<String>()}) => LogType::Repair ; "repair")]
    #[test_case::test_case(json!({"log_type": "wiper_blade_replacement", "extra_field": Faker.fake::<String>()}) => LogType::WiperBladeReplacement ; "wiper_blade_replacement")]
    fn unit_types_deserialize_correctly_ignoring_extra_fields(value: serde_json::Value) -> LogType {
        // Act
        serde_json::from_value(value).expect("could not deserialize")
    }

    #[test_case::test_case(json!({"log_type": "fuel_up"}) ; "fuel_up")]
    #[test_case::test_case(json!({"log_type": "tire_rotation"}) ; "tire_rotation")]
    #[test_case::test_case(json!({"log_type": "tire_change"}) ; "tire_change")]
    #[test_case::test_case(json!({"log_type": "brake_replacement"}) ; "brake_replacement")]
    fn deserializing_fails_with_missing_fields(value: serde_json::Value) {
        // Arrange

        // Act
        let err = serde_json::from_value::<LogType>(value)
            .expect_err("deserialization didn't fail as expected");

        // Assert
        assert!(err.is_data());
    }
}
