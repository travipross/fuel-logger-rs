use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    types::vehicle::{
        api::{
            CreateVehicleResponse, DeleteVehicleResponse, ListVehiclesResponse,
            ReadVehicleResponse, UpdateVehicleResponse,
        },
        db::Vehicle,
    },
};

pub async fn read(pool: &PgPool, id: Uuid) -> Result<ReadVehicleResponse, ApiError> {
    let sql = "SELECT * FROM vehicles WHERE id = $1";

    let vehicle = query_as::<_, Vehicle>(sql).bind(id).fetch_one(pool).await?;

    vehicle.try_into()
}

pub async fn list(pool: &PgPool) -> Result<ListVehiclesResponse, ApiError> {
    let sql = "SELECT * FROM vehicles";
    let vehicles = sqlx::query_as::<_, Vehicle>(sql).fetch_all(pool).await?;

    vehicles.into_iter().map(TryInto::try_into).collect()
}

pub async fn create(pool: &PgPool, vehicle: Vehicle) -> Result<CreateVehicleResponse, ApiError> {
    let sql = "
        INSERT INTO vehicles (
            make, 
            model, 
            year, 
            odometer_unit
        ) VALUES (
            $1, 
            $2, 
            $3, 
            $4
        ) RETURNING id";

    let res = query(sql)
        .bind(vehicle.make)
        .bind(vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.odometer_unit)
        .fetch_one(pool)
        .await?;

    let id = res.try_get::<Uuid, _>("id")?;

    Ok(CreateVehicleResponse { id })
}

pub async fn update(
    pool: &PgPool,
    vehicle_id: Uuid,
    vehicle: Vehicle,
) -> Result<UpdateVehicleResponse, ApiError> {
    let sql = "
        UPDATE vehicles 
        SET 
            make = $1, 
            model = $2, 
            year = $3, 
            odometer_unit = $4
        WHERE id = $5 
        RETURNING *";
    let updated_vehicle = query_as::<_, Vehicle>(sql)
        .bind(vehicle.make)
        .bind(vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.odometer_unit)
        .bind(vehicle_id)
        .fetch_one(pool)
        .await?;

    updated_vehicle.try_into()
}

pub async fn delete(pool: &PgPool, vehicle_id: Uuid) -> Result<DeleteVehicleResponse, ApiError> {
    let sql = "DELETE FROM vehicles where id = $1";
    let res = query(sql).bind(vehicle_id).execute(pool).await?;
    if res.rows_affected() < 1 {
        Err(ApiError::ResourceNotFound)
    } else {
        Ok(DeleteVehicleResponse)
    }
}
