use axum::{
    extract::Path,
    routing::{get, post, put},
    Json, Router,
};
use fake::{Fake, Faker};
use uuid::Uuid;

use crate::models::{Vehicle, VehicleInput};

async fn list() -> Json<Vec<Vehicle>> {
    let vehicles = Faker.fake::<Vec<Vehicle>>();
    Json(vehicles)
}

async fn read(Path(vehicle_id): Path<Uuid>) -> Json<Vehicle> {
    println!("Getting vehicle with ID: {}", vehicle_id);
    let vehicle = Faker.fake::<Vehicle>();
    Json(vehicle)
}

async fn create(Json(vehicle_input): Json<VehicleInput>) -> Json<Vehicle> {
    let vehicle = Vehicle {
        id: Faker.fake(),
        owner_id: Faker.fake(),
        make: vehicle_input.make,
        model: vehicle_input.model,
        year: vehicle_input.year,
        odometer_unit: vehicle_input.odometer_unit.unwrap_or_default(),
    };
    println!("Create vehicle: {vehicle:?}");
    Json(vehicle)
}

async fn update(
    Path(vehicle_id): Path<Uuid>,
    Json(vehicle_input): Json<VehicleInput>,
) -> Json<Vehicle> {
    let vehicle = Vehicle {
        id: vehicle_id,
        owner_id: Faker.fake(),
        make: vehicle_input.make,
        model: vehicle_input.model,
        year: vehicle_input.year,
        odometer_unit: vehicle_input.odometer_unit.unwrap_or_default(),
    };
    println!("Updated vehicle: {vehicle:?}");
    Json(vehicle)
}

pub fn build_router() -> Router {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:vehicle_id", get(read))
        .route("/:vehicle_id", put(update))
}
