use crate::types::primitives::{
    BrakeComponent, BrakeLocation, FluidType, TireRotationType, TireType,
};

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, fake::Dummy)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "log_type")]
pub enum LogType {
    FuelUp {
        #[dummy(faker = "5.0..120.0")]
        fuel_amount: f32,
    },
    TireRotation(TireRotationType),
    TireChange {
        #[serde(flatten)]
        rotation: Option<TireRotationType>,
        tire_type: TireType,
        new: bool,
    },
    OilChange,
    Repair,
    WiperBladeReplacement,
    BatteryReplacement,
    BrakeReplacement {
        location: BrakeLocation,
        #[serde(rename = "brake_part")]
        component: BrakeComponent,
    },
    Fluids(FluidType),
}

impl std::fmt::Display for LogType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FuelUp { .. } => write!(f, "fuel_up"),
            Self::TireRotation(_) => write!(f, "tire_rotation"),
            Self::TireChange { .. } => write!(f, "tire_change"),
            Self::OilChange => write!(f, "oil_change"),
            Self::Repair => write!(f, "repair"),
            Self::WiperBladeReplacement => write!(f, "wiper_blade_replacement"),
            Self::BatteryReplacement => write!(f, "battery_replacement"),
            Self::BrakeReplacement { .. } => write!(f, "brake_replacement"),
            Self::Fluids(_) => write!(f, "fluids"),
        }
    }
}
