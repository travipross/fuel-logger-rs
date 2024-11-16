use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;

use crate::{
    controllers::vehicle::{
        create as create_vehicle, delete as delete_vehicle, list as list_vehicles,
        read as read_vehicle, update as update_vehicle,
    },
    error::ApiError,
    types::vehicle::{
        api::{
            CreateVehicleBody, CreateVehicleResponse, DeleteVehicleResponse, ListVehiclesResponse,
            ReadVehicleResponse, UpdateVehicleBody, UpdateVehicleResponse,
        },
        db::Vehicle as DbVehicle,
    },
    AppState,
};

async fn list(State(appstate): State<AppState>) -> Result<ListVehiclesResponse, ApiError> {
    list_vehicles(&appstate.db).await
}

async fn read(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
) -> Result<ReadVehicleResponse, ApiError> {
    read_vehicle(&appstate.db, vehicle_id).await
}

async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateVehicleBody>,
) -> Result<CreateVehicleResponse, ApiError> {
    let db_vehicle = DbVehicle::from_api_type(&Uuid::new_v4(), body);
    create_vehicle(&appstate.db, db_vehicle).await
}

async fn update(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
    Json(body): Json<UpdateVehicleBody>,
) -> Result<UpdateVehicleResponse, ApiError> {
    let db_vehicle = DbVehicle::from_api_type(&vehicle_id, body);
    update_vehicle(&appstate.db, vehicle_id, db_vehicle).await
}

async fn delete_route(
    Path(vehicle_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Result<DeleteVehicleResponse, ApiError> {
    delete_vehicle(&appstate.db, vehicle_id).await
}

pub fn build_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:vehicle_id", get(read))
        .route("/:vehicle_id", put(update))
        .route("/:vehicle_id", delete(delete_route))
        .nest(
            "/:vehicle_id/log_records",
            super::log_records::build_router(),
        )
}
