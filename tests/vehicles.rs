mod common;

use axum::http::StatusCode;
use common::{seed_user, seed_vehicle, seed_vehicle_and_user};
use fake::{
    faker::company::en::{Buzzword, CompanyName},
    Fake, Faker,
};
use fuel_logger_rs::models::DbVehicle;
use serde_json::json;
use sqlx::{query_as, PgPool, Row};
use uuid::Uuid;

#[sqlx::test]
async fn create(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user = seed_user(&pool).await;
    let make = CompanyName().fake::<String>();
    let model = Buzzword().fake::<String>();
    let year = (1950..2030).fake::<i32>();
    let input = json!({
        "owner_id": user.id,
        "make": make,
        "model": model,
        "year": year,
    });

    // Act
    let res = server.post("/vehicles").json(&input).await;
    let created_vehicle_id =
        sqlx::query("SELECT id FROM vehicles WHERE make = $1 AND model = $2 AND year = $3 LIMIT 1")
            .bind(make)
            .bind(model)
            .bind(year)
            .fetch_one(&pool)
            .await
            .expect("could not fetch from database")
            .get::<Uuid, _>("id");

    // Assert
    res.assert_status(StatusCode::CREATED);
    assert_eq!(
        res.header("location"),
        format!("/vehicles/{created_vehicle_id}")
    );
    res.assert_json_contains(&json!({"id": created_vehicle_id}));
}

#[sqlx::test]
async fn read(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;

    // Act
    let res = server
        .get(format!("/vehicles/{}", vehicle.id).as_str())
        .await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!({
        "id": vehicle.id,
        "make": vehicle.make,
        "model": vehicle.model,
        "year": vehicle.year,
        "odometer_unit": vehicle.odometer_unit,
    }));
}

#[sqlx::test]
async fn list(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user = seed_user(&pool).await;
    let vehicle_1 = seed_vehicle(&pool, user.id).await;
    let vehicle_2 = seed_vehicle(&pool, user.id).await;

    // Act
    let res = server.get("/vehicles").await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!([
    {
        "id": vehicle_1.id,
        "make": vehicle_1.make,
        "model": vehicle_1.model,
        "year": vehicle_1.year,
        "odometer_unit": vehicle_1.odometer_unit,
    },
    {
        "id": vehicle_2.id,
        "make": vehicle_2.make,
        "model": vehicle_2.model,
        "year": vehicle_2.year,
        "odometer_unit": vehicle_2.odometer_unit,
    },
    ]));
}

#[sqlx::test]
async fn update(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    let updated_vehicle = DbVehicle {
        id: vehicle.id,
        owner_id: vehicle.owner_id,
        ..Faker.fake()
    };
    let update_body = json!({
        "id": updated_vehicle.id,
        "owner_id": updated_vehicle.owner_id,
        "make": updated_vehicle.make,
        "model": updated_vehicle.model,
        "year": updated_vehicle.year,
        "odometer_unit": updated_vehicle.odometer_unit,
    });

    // Act
    let res = server
        .put(format!("/vehicles/{}", vehicle.id).as_str())
        .json(&update_body)
        .await;
    let written_vehicle = query_as::<_, DbVehicle>(
        "SELECT * FROM vehicles WHERE make = $1 AND model = $2 AND year = $3 LIMIT 1",
    )
    .bind(updated_vehicle.make.clone())
    .bind(updated_vehicle.model.clone())
    .bind(updated_vehicle.year)
    .fetch_one(&pool)
    .await
    .expect("could not read vehicle from db");

    // Assert
    res.assert_status(StatusCode::OK);
    assert_eq!(written_vehicle, updated_vehicle);
    res.assert_json_contains(&json!({
        "id": updated_vehicle.id,
        "make": updated_vehicle.make,
        "model": updated_vehicle.model,
        "year": updated_vehicle.year,
        "odometer_unit": updated_vehicle.odometer_unit,
    }));
}

#[sqlx::test]
async fn delete(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let vehicle = seed_vehicle_and_user(&pool).await;
    assert!(
        query_as::<_, DbVehicle>("SELECT * FROM vehicles WHERE id = $1 LIMIT 1")
            .bind(vehicle.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read vehicle from db")
            .is_some()
    );

    // Act
    let res = server
        .delete(format!("/vehicles/{}", vehicle.id).as_str())
        .await;

    // Assert
    res.assert_status(StatusCode::NO_CONTENT);
    assert!(res.into_bytes().is_empty());
    assert!(
        query_as::<_, DbVehicle>("SELECT * FROM vehicles WHERE id = $1 LIMIT 1")
            .bind(vehicle.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read vehicle from db")
            .is_none()
    );
}
