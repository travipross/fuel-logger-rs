#[allow(dead_code)]
#[derive(Debug)]
pub enum LogType {
    FuelUp { amount: f32 },
    // TireRotation,
    // TireChange,
    // OilChange,
    // Repair,
    // WiperBlades,
    // BatteryReplacement,
}

#[derive(Debug)]
pub struct LogRecord {
    pub date: String,
    pub log_type: LogType,
    pub odometer: u32,
}
