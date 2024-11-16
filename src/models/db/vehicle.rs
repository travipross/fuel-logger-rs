use uuid::Uuid;

use crate::{
    models::api::CreateVehicleBody as ApiCreateVehicleBody, types::primitives::OdometerUnit,
};

#[derive(Debug, PartialEq, serde::Deserialize, fake::Dummy, sqlx::FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub owner_id: Uuid,
    #[dummy(faker = "fake::faker::company::en::CompanyName()")]
    pub make: String,
    #[dummy(faker = "fake::faker::company::en::Buzzword()")]
    pub model: String,
    #[dummy(faker = "1950..2030")]
    pub year: i32,
    #[serde(skip)]
    pub odometer_unit: OdometerUnit,
}

impl Vehicle {
    pub fn from_api_type(vehicle_id: &Uuid, body: ApiCreateVehicleBody) -> Self {
        Self {
            id: *vehicle_id,
            owner_id: body.owner_id,
            make: body.make,
            model: body.model,
            year: body.year.into(),
            odometer_unit: body.odometer_unit.unwrap_or_default(),
        }
    }
}
