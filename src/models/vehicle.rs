use crate::models::{LogRecord, Person};
use fake::Dummy;
use serde::Serialize;

use super::OdometerKms;

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub enum OdometerUnit {
    Metric,
    Imperial,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub struct Vehicle {
    make: String,
    model: String,
    year: u16,
    owner: Person,
    odometer_unit: OdometerUnit,
    logs: Vec<LogRecord>,
}

impl Vehicle {
    pub fn new(
        make: String,
        model: String,
        year: u16,
        owner: Person,
        odometer_unit: OdometerUnit,
    ) -> Self {
        Vehicle {
            make,
            model,
            year,
            owner,
            odometer_unit,
            logs: vec![],
        }
    }

    pub fn add_record(&mut self, record: LogRecord) {
        self.logs.push(record);
    }

    pub fn get_current_odo(&mut self) -> OdometerKms {
        self.logs.sort_by_key(|l| l.odometer);
        match self.logs.last() {
            Some(record) => record.odometer,
            None => OdometerKms(0),
        }
    }
}
