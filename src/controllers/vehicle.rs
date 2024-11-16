use sqlx::{query, query_as, PgPool, Row};
use uuid::Uuid;

use crate::{
    error::ApiError,
    models::{
        api::{
            CreateVehicleBody, CreateVehicleResponse, DeleteVehicleResponse, ListVehiclesResponse,
            ReadVehicleResponse, UpdateVehicleBody, UpdateVehicleResponse,
        },
        db::Vehicle as DbVehicle,
    },
};

pub async fn read(pool: &PgPool, id: &Uuid) -> Result<ReadVehicleResponse, ApiError> {
    let sql = "SELECT * FROM vehicles WHERE id = $1";

    let vehicle = query_as::<_, DbVehicle>(sql)
        .bind(id)
        .fetch_one(pool)
        .await?;

    vehicle.try_into()
}

pub async fn list(pool: &PgPool) -> Result<ListVehiclesResponse, ApiError> {
    let sql = "SELECT * FROM vehicles";
    let vehicles = sqlx::query_as::<_, DbVehicle>(sql).fetch_all(pool).await?;

    vehicles.into_iter().map(TryInto::try_into).collect()
}

pub async fn create(
    pool: &PgPool,
    body: CreateVehicleBody,
) -> Result<CreateVehicleResponse, ApiError> {
    let vehicle = DbVehicle::from_api_type(&Uuid::new_v4(), body);
    let sql = "
        INSERT INTO vehicles (
            owner_id,
            make, 
            model, 
            year, 
            odometer_unit
        ) VALUES (
            $1, 
            $2, 
            $3, 
            $4,
            $5
        ) RETURNING id";

    let res = query(sql)
        .bind(vehicle.owner_id)
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
    vehicle_id: &Uuid,
    body: UpdateVehicleBody,
) -> Result<UpdateVehicleResponse, ApiError> {
    let vehicle = DbVehicle::from_api_type(vehicle_id, body);
    let sql = "
        UPDATE vehicles 
        SET 
            make = $1, 
            model = $2, 
            year = $3, 
            odometer_unit = $4
        WHERE id = $5
        RETURNING *";
    let updated_vehicle = query_as::<_, DbVehicle>(sql)
        .bind(vehicle.make)
        .bind(vehicle.model)
        .bind(vehicle.year)
        .bind(vehicle.odometer_unit)
        .bind(vehicle.id)
        .fetch_one(pool)
        .await?;

    updated_vehicle.try_into()
}

pub async fn delete(pool: &PgPool, vehicle_id: &Uuid) -> Result<DeleteVehicleResponse, ApiError> {
    let sql = "DELETE FROM vehicles WHERE id = $1 RETURNING *";
    Ok(query(sql)
        .bind(vehicle_id)
        .fetch_one(pool)
        .await
        .map(|_| DeleteVehicleResponse)?)
}
