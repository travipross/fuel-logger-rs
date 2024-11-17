#![allow(dead_code)]
use fake::{Fake, Faker};
use fuel_logger_rs::models::{DbLogRecord, DbUser, DbVehicle};
use sqlx::{query_as, PgPool};
use uuid::Uuid;

pub async fn seed_user(pool: &PgPool) -> DbUser {
    let user = Faker.fake::<DbUser>();
    query_as::<_, DbUser>(
        "
        INSERT INTO users (
            id, 
            first_name, 
            last_name, 
            username, 
            email
        ) VALUES (
            $1, 
            $2, 
            $3, 
            $4, 
            $5
        ) RETURNING *",
    )
    .bind(user.id)
    .bind(&user.first_name)
    .bind(&user.last_name)
    .bind(&user.username)
    .bind(&user.email)
    .fetch_one(pool)
    .await
    .expect("could not seed user record")
}

pub async fn seed_vehicle(pool: &PgPool, owner_id: Uuid) -> DbVehicle {
    let vehicle = DbVehicle {
        owner_id,
        ..Faker.fake()
    };
    query_as::<_, DbVehicle>(
        "
        INSERT INTO vehicles (
            id, 
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
            $5, 
            $6
        ) RETURNING *",
    )
    .bind(vehicle.id)
    .bind(vehicle.owner_id)
    .bind(vehicle.make)
    .bind(vehicle.model)
    .bind(vehicle.year)
    .bind(vehicle.odometer_unit)
    .fetch_one(pool)
    .await
    .expect("could not seed vehicle record")
}

pub async fn seed_vehicle_and_user(pool: &PgPool) -> DbVehicle {
    let user = seed_user(pool).await;

    seed_vehicle(pool, user.id).await
}

pub async fn seed_log_record_and_vehicle(pool: &PgPool) -> DbLogRecord {
    let vehicle = seed_vehicle_and_user(pool).await;
    seed_log_record(pool, vehicle.id).await
}

pub async fn seed_log_record(pool: &PgPool, vehicle_id: Uuid) -> DbLogRecord {
    let log_record = DbLogRecord {
        vehicle_id,
        ..Faker.fake()
    };

    write_log_record(pool, log_record).await
}

pub async fn write_log_record(pool: &PgPool, log_record: DbLogRecord) -> DbLogRecord {
    query_as::<_, DbLogRecord>(
        "
        INSERT INTO log_records (
            id,
            vehicle_id,
            log_date,
            odometer,
            log_type,
            fuel_amount,
            notes,
            tire_rotation_type,
            tire_type,
            new_tires,
            brake_location,
            brake_part,
            fluid_type
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9,
            $10,
            $11,
            $12,
            $13
        ) RETURNING *
    ",
    )
    .bind(log_record.id)
    .bind(log_record.vehicle_id)
    .bind(log_record.date)
    .bind(log_record.odometer)
    .bind(log_record.log_type())
    .bind(log_record.fuel_amount())
    .bind(&log_record.notes)
    .bind(log_record.tire_rotation_type())
    .bind(log_record.tire_type())
    .bind(log_record.new_tires())
    .bind(log_record.brake_location())
    .bind(log_record.brake_part())
    .bind(log_record.fluid_type())
    .fetch_one(pool)
    .await
    .expect("could not write log_record")
}
