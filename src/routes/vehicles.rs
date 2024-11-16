use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    controllers::{
        create as create_vehicle, delete as delete_vehicle, list as list_vehicles,
        read as read_vehicle, update as update_vehicle,
    },
    models::{CreateVehicleResponse, Vehicle, VehicleInput},
    AppState, DEFAULT_USER_ID,
};

async fn list(State(appstate): State<AppState>) -> Json<Vec<Vehicle>> {
    let vehicles = list_vehicles(&appstate.db)
        .await
        .expect("could not list vehicles");
    Json(vehicles)
}

async fn read(State(appstate): State<AppState>, Path(vehicle_id): Path<Uuid>) -> Json<Vehicle> {
    println!("Getting vehicle with ID: {}", vehicle_id);
    let vehicle = read_vehicle(&appstate.db, vehicle_id).await.unwrap();
    Json(vehicle)
}

async fn create(
    State(appstate): State<AppState>,
    Json(vehicle_input): Json<VehicleInput>,
) -> Json<CreateVehicleResponse> {
    let default_user_id = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
    let response = create_vehicle(&appstate.db, vehicle_input.into_db_input(&default_user_id))
        .await
        .unwrap();
    Json(response)
}

async fn update(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
    Json(vehicle_input): Json<VehicleInput>,
) -> Json<Vehicle> {
    let default_user_id = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
    let response = update_vehicle(
        &appstate.db,
        vehicle_id,
        vehicle_input.into_db_input(&default_user_id),
    )
    .await
    .unwrap();
    Json(response)
}

async fn delete_route(
    Path(vehicle_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Json<serde_json::Value> {
    delete_vehicle(&appstate.db, vehicle_id).await.unwrap();
    Json(json!({}))
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:vehicle_id", get(read))
        .route("/:vehicle_id", put(update))
        .route("/:vehicle_id", delete(delete_route))
}
