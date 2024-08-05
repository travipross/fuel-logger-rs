use std::str::FromStr;

use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use fake::{Fake, Faker};
use uuid::Uuid;

use crate::{
    models::{Vehicle, VehicleInput},
    AppState,
};

async fn list() -> Json<Vec<Vehicle>> {
    let vehicles = Faker.fake::<Vec<Vehicle>>();
    Json(vehicles)
}

async fn read(Path(vehicle_id): Path<Uuid>) -> Json<Vehicle> {
    println!("Getting vehicle with ID: {}", vehicle_id);
    let vehicle = Faker.fake::<Vehicle>();
    Json(vehicle)
}

async fn create(
    State(appstate): State<AppState>,
    Json(vehicle_input): Json<VehicleInput>,
) -> Json<Vehicle> {
    let default_user_id = Uuid::from_str("50c5ab2e-4c29-4583-a698-5902b861b628").unwrap();

    // let new_vehicle = sqlx::query_as!(
    //     VehicleDb,
    //     "INSERT INTO vehicles (owner_id, make, model, year, odometer_unit) VALUES ($1, $2, $3, $4, $5)",
    //     default_user_id,
    //     db_vehicle.make,
    //     db_vehicle.model,
    //     db_vehicle.year,
    //     db_vehicle.odometer_unit
    // )
    //     .fetch_one(&appstate.db)
    //     .await?
    //     ;

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

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:vehicle_id", get(read))
        .route("/:vehicle_id", put(update))
}
