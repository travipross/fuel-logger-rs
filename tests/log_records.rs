use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use common::{
    db::write_log_record, seed_log_record, seed_log_record_and_vehicle, seed_vehicle_and_user,
};
use fake::{Fake, Faker};
use fuel_logger_rs::{models::DbLogRecord, types::LogType};
use serde_json::json;
use sqlx::{query_as, PgPool, Row};
use uuid::Uuid;

mod common;

#[sqlx::test]
async fn create(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    let odometer = (100..100000).fake::<i32>();
    let input = json!({
        "vehicle_id": vehicle.id,
        "odometer": odometer,
        "date": Faker.fake::<DateTime<Utc>>(),
        "notes": Faker.fake::<Option<String>>(),
        "fuel_amount": Faker.fake::<f32>(),
        "log_type": "fuel_up",
    });

    // Act
    let res = server.post("/log_records").json(&input).await;
    let created_log_record_id =
        sqlx::query("SELECT id FROM log_records WHERE odometer = $1 LIMIT 1")
            .bind(odometer)
            .fetch_one(&pool)
            .await
            .expect("could not fetch from database")
            .get::<Uuid, _>("id");

    // Assert
    res.assert_status(StatusCode::CREATED);
    assert_eq!(
        res.header("location"),
        format!("/log_records/{created_log_record_id}")
    );
    res.assert_json_contains(&json!({"id": created_log_record_id}));
}

#[sqlx::test]
async fn read(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let log_record = seed_log_record_and_vehicle(&pool).await;

    // Act
    let res = server
        .get(format!("/log_records/{}", log_record.id).as_str())
        .await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!({
        "id": log_record.id,
        "odometer": log_record.odometer,
        "date": log_record.date,
        "log_type": log_record.log_type(),
        "notes": log_record.notes,
    }));
}

#[sqlx::test]
async fn list(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    let log_record_1 = seed_log_record(&pool, vehicle.id).await;
    let log_record_2 = seed_log_record(&pool, vehicle.id).await;

    // Act
    let res = server.get("/log_records").await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!([
    {
        "id": log_record_1.id,
        "odometer": log_record_1.odometer,
        "date": log_record_1.date,
        "log_type": log_record_1.log_type(),
        "notes": log_record_1.notes,
    },
    {
        "id": log_record_2.id,
        "odometer": log_record_2.odometer,
        "date": log_record_2.date,
        "log_type": log_record_2.log_type(),
        "notes": log_record_2.notes,
    }
    ]));
}

#[sqlx::test]
async fn update(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    let initial_log_record = DbLogRecord {
        log_type: LogType::FuelUp {
            fuel_amount: Faker.fake(),
        },
        vehicle_id: vehicle.id,
        ..Faker.fake()
    };

    let log_record = write_log_record(&pool, initial_log_record).await;

    let updated_log_record = DbLogRecord {
        id: log_record.id,
        vehicle_id: log_record.vehicle_id,
        log_type: log_record.log_type,
        ..Faker.fake()
    };
    let update_body = json!({
        "vehicle_id": updated_log_record.vehicle_id,
        "odometer": updated_log_record.odometer,
        "date": updated_log_record.date,
        "notes": updated_log_record.notes,
        "log_type": updated_log_record.log_type(),
        "fuel_amount": updated_log_record.fuel_amount(),
    });

    // Act
    let res = server
        .put(format!("/log_records/{}", log_record.id).as_str())
        .json(&update_body)
        .await;
    let written_log_record =
        query_as::<_, DbLogRecord>("SELECT * FROM log_records WHERE id = $1 LIMIT 1")
            .bind(updated_log_record.id)
            .fetch_one(&pool)
            .await
            .expect("could not read vehicle from db");

    // Assert
    res.assert_status(StatusCode::OK);
    assert_eq!(written_log_record, updated_log_record);
    res.assert_json_contains(&json!({
        "id": updated_log_record.id,
        "vehicle_id": updated_log_record.vehicle_id,
        "odometer": updated_log_record.odometer,
        "notes": updated_log_record.notes,
    }));
}

#[sqlx::test]
async fn update_wrong_type(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    let initial_log_record = DbLogRecord {
        log_type: LogType::FuelUp {
            fuel_amount: Faker.fake(),
        },
        vehicle_id: vehicle.id,
        ..Faker.fake()
    };

    let log_record = write_log_record(&pool, initial_log_record).await;

    let updated_log_record = DbLogRecord {
        id: log_record.id,
        vehicle_id: log_record.vehicle_id,
        log_type: LogType::BatteryReplacement,
        ..Faker.fake()
    };
    let update_body = json!({
        "vehicle_id": updated_log_record.vehicle_id,
        "odometer": updated_log_record.odometer,
        "date": updated_log_record.date,
        "notes": updated_log_record.notes,
        "log_type": updated_log_record.log_type(),
    });

    // Act
    let res = server
        .put(format!("/log_records/{}", log_record.id).as_str())
        .json(&update_body)
        .await;

    // Assert
    res.assert_status(StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn delete(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let log_record = seed_log_record_and_vehicle(&pool).await;
    assert!(
        query_as::<_, DbLogRecord>("SELECT * FROM log_records WHERE id = $1 LIMIT 1")
            .bind(log_record.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read log_record from db")
            .is_some()
    );

    // Act
    let res = server
        .delete(format!("/log_records/{}", log_record.id).as_str())
        .await;

    // Assert
    res.assert_status(StatusCode::NO_CONTENT);
    assert!(res.into_bytes().is_empty());
    assert!(
        query_as::<_, DbLogRecord>("SELECT * FROM log_records WHERE id = $1 LIMIT 1")
            .bind(log_record.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read vehicle from db")
            .is_none()
    );
}
