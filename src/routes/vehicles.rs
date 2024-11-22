use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

use crate::{
    controllers::vehicle::{
        create as create_vehicle, delete as delete_vehicle, list as list_vehicles,
        read as read_vehicle, update as update_vehicle,
    },
    error::ApiError,
    extractors::custom_json::Json,
    models::api::{
        CreateVehicleBody, CreateVehicleResponse, DeleteVehicleResponse, ListVehiclesResponse,
        ReadVehicleResponse, UpdateVehicleBody, UpdateVehicleResponse,
    },
    AppState,
};

#[tracing::instrument(name = "vehicles_list_route", skip(appstate), err)]
async fn list(State(appstate): State<AppState>) -> Result<ListVehiclesResponse, ApiError> {
    list_vehicles(&appstate.db).await
}

#[tracing::instrument(name = "vehicles_read_route", skip(appstate), err)]
async fn read(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
) -> Result<ReadVehicleResponse, ApiError> {
    read_vehicle(&appstate.db, &vehicle_id).await
}

#[tracing::instrument(name = "vehicles_create_route", skip(appstate), err)]
async fn create(
    State(appstate): State<AppState>,
    Json(body): Json<CreateVehicleBody>,
) -> Result<CreateVehicleResponse, ApiError> {
    create_vehicle(&appstate.db, body).await
}

#[tracing::instrument(name = "vehicles_update_route", skip(appstate), err)]
async fn update(
    State(appstate): State<AppState>,
    Path(vehicle_id): Path<Uuid>,
    Json(body): Json<UpdateVehicleBody>,
) -> Result<UpdateVehicleResponse, ApiError> {
    update_vehicle(&appstate.db, &vehicle_id, body).await
}

#[tracing::instrument(name = "vehicles_delete_route", skip(appstate), err)]
async fn delete_route(
    Path(vehicle_id): Path<Uuid>,
    State(appstate): State<AppState>,
) -> Result<DeleteVehicleResponse, ApiError> {
    delete_vehicle(&appstate.db, &vehicle_id).await
}

#[tracing::instrument(name = "build_vehicles_router", skip_all)]
pub fn build_router() -> Router<AppState> {
    tracing::debug!("building vehicles router");
    Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/:vehicle_id", get(read))
        .route("/:vehicle_id", put(update))
        .route("/:vehicle_id", delete(delete_route))
}
