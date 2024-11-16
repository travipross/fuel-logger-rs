use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    controllers::vehicle::{
        create as create_vehicle, delete as delete_vehicle, list as list_vehicles,
        read as read_vehicle, update as update_vehicle,
    },
    types::vehicle::{
        api::{
            CreateVehicleBody, CreateVehicleResponse, ReadVehicleResponse, UpdateVehicleBody,
            UpdateVehicleResponse,
        },
        db::Vehicle as DbVehicle,
    },
    AppState, DEFAULT_USER_ID,
};

async fn list(State(appstate): State<AppState>) -> Json<Vec<ReadVehicleResponse>> {
    let vehicles = list_vehicles(&appstate.db)
        .await
        .expect("could not list vehicles");
    Json(vehicles)
}

async fn read(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
) -> Json<ReadVehicleResponse> {
    println!("Getting vehicle with ID: {}", vehicle_id);
    let vehicle = read_vehicle(&appstate.db, vehicle_id).await.unwrap();
    Json(vehicle)
}

async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateVehicleBody>,
) -> Json<CreateVehicleResponse> {
    let default_user_id = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
    let db_vehicle = DbVehicle::from_api_type(&Uuid::new_v4(), &default_user_id, body);
    let response = create_vehicle(&appstate.db, db_vehicle).await.unwrap();
    Json(response)
}

async fn update(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
    Json(body): Json<UpdateVehicleBody>,
) -> Json<UpdateVehicleResponse> {
    let default_user_id = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
    let db_vehicle = DbVehicle::from_api_type(&vehicle_id, &default_user_id, body);
    let response = update_vehicle(&appstate.db, vehicle_id, db_vehicle)
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
