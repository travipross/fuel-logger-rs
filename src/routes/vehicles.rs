use axum::Json;
use fake::{Fake, Faker};

use crate::models::Vehicle;

pub async fn get_vehicles() -> Json<Vehicle> {
    let vehicle1 = Faker.fake::<Vehicle>();
    Json(vehicle1)
}
