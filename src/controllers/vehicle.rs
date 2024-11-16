use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{CreateVehicle, CreateVehicleResponse, Vehicle},
};

pub async fn read(pool: &PgPool, id: Uuid) -> Result<Vehicle, ApiError> {
    let sql = "SELECT * FROM vehicles WHERE id = $1";

    let vehicle = query_as::<_, Vehicle>(sql).bind(id).fetch_one(pool).await?;

    Ok(vehicle)
}

pub async fn list(pool: &PgPool) -> Result<Vec<Vehicle>, ApiError> {
    let sql = "SELECT * FROM vehicles";
    let vehicles = sqlx::query_as::<_, Vehicle>(sql).fetch_all(pool).await?;
    Ok(vehicles)
}

pub async fn create(
    pool: &PgPool,
    create_vehicle_input: CreateVehicle,
) -> Result<CreateVehicleResponse, ApiError> {
    let sql = "INSERT INTO vehicles (make, model, year, owner_id, odometer_unit) VALUES ($1, $2, $3, $4, $5) RETURNING id";
    let res = query(sql)
        .bind(create_vehicle_input.make)
        .bind(create_vehicle_input.model)
        .bind(create_vehicle_input.year as i32)
        .bind(create_vehicle_input.owner_id)
        .bind(create_vehicle_input.odometer_unit)
        .fetch_one(pool)
        .await?;

    let id = res.try_get::<Uuid, _>("id")?;

    Ok(CreateVehicleResponse { id })
}

pub async fn update(
    pool: &PgPool,
    vehicle_id: Uuid,
    update_vehicle_input: CreateVehicle,
) -> Result<Vehicle, ApiError> {
    let sql = "
        UPDATE vehicles 
        SET 
            make = $1, 
            model = $2, 
            year = $3, 
            owner_id = $4, 
            odometer_unit = $5 
        WHERE id = $6 
        RETURNING *";
    let updated_vehicle = query_as::<_, Vehicle>(sql)
        .bind(update_vehicle_input.make)
        .bind(update_vehicle_input.model)
        .bind(update_vehicle_input.year as i32)
        .bind(update_vehicle_input.owner_id)
        .bind(update_vehicle_input.odometer_unit)
        .bind(vehicle_id)
        .fetch_one(pool)
        .await?;
    Ok(updated_vehicle)
}

pub async fn delete(pool: &PgPool, vehicle_id: Uuid) -> Result<(), ApiError> {
    let sql = "DELETE FROM vehicles where id = $1";
    query(sql).bind(vehicle_id).execute(pool).await?;
    Ok(())
}
