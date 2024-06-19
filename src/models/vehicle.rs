use crate::models::{LogRecord, Person};
use fake::Dummy;
use serde::Serialize;

use super::OdometerKms;
use fake::faker::company::en::{Buzzword, CompanyName};

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub enum OdometerUnit {
    Metric,
    Imperial,
}

#[derive(Debug, Serialize, Dummy)]
pub struct VehicleMake(#[dummy(faker = "CompanyName()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct VehicleModel(#[dummy(faker = "Buzzword()")] pub String);

#[derive(Debug, Serialize, Dummy)]
pub struct VehicleYear(#[dummy(faker = "1950..2030")] pub u16);

#[allow(dead_code)]
#[derive(Debug, Serialize, Dummy)]
pub struct Vehicle {
    make: VehicleMake,
    model: VehicleModel,
    year: VehicleYear,
    owner: Person,
    odometer_unit: OdometerUnit,
    logs: Vec<LogRecord>,
}

impl Vehicle {
    pub fn new(
        make: VehicleMake,
        model: VehicleModel,
        year: VehicleYear,
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
