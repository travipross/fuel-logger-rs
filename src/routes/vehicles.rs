use axum::{extract::Path, Json};
use fake::{Fake, Faker};

use crate::models::Vehicle;

pub async fn get_vehicles() -> Json<Vec<Vehicle>> {
    let vehicles = Faker.fake::<Vec<Vehicle>>();
    Json(vehicles)
}

pub async fn get_vehicle(Path(vehicle_id): Path<u32>) -> Json<Vehicle> {
    println!("Getting vehicle with ID: {}", vehicle_id);
    let vehicle = Faker.fake::<Vehicle>();
    Json(vehicle)
}
