use crate::models::{LogRecord, Person};

#[allow(dead_code)]
#[derive(Debug)]
pub enum OdometerUnit {
    Metric,
    Imperial,
}

#[derive(Debug)]
pub struct Vehicle {
    pub year: u16,
    pub make: String,
    pub model: String,
    pub owner: Person,
    pub odometer_unit: OdometerUnit,
    pub logs: Vec<LogRecord>,
}
