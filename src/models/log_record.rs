use std::fmt::Display;

use chrono::{DateTime, Utc};
use fake::Dummy;
use serde::Serialize;

#[derive(Debug, Serialize, Dummy, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct OdometerKms(#[dummy(faker = "0..500000")] pub u32);

impl Display for OdometerKms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Dummy)]
pub struct FuelAmountLitres(#[dummy(faker = "5.0..120.0")] pub f32);

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub enum LogType {
    FuelUp { amount: FuelAmountLitres },
    // TireRotation,
    // TireChange,
    // OilChange,
    // Repair,
    // WiperBlades,
    // BatteryReplacement,
}

#[derive(Debug, Serialize, Dummy)]
pub struct LogRecord {
    pub date: DateTime<Utc>,
    pub log_type: LogType,
    pub odometer: OdometerKms,
}
