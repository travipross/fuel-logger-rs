mod common;

use axum::http::StatusCode;
use common::seed_user;
use fake::{
    faker::{
        internet::en::{FreeEmail, Username},
        name::en::{FirstName, LastName},
    },
    Fake, Faker,
};
use fuel_logger_rs::models::DbUser;
use serde_json::json;
use sqlx::{query_as, PgPool, Row};
use uuid::Uuid;

#[sqlx::test]
async fn create(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let username = Username().fake::<String>();
    let input = json!({
        "first_name": FirstName().fake::<String>(),
        "last_name": LastName().fake::<String>(),
        "username": username,
        "email": FreeEmail().fake::<String>(),
    });

    // Act
    let res = server.post("/users").json(&input).await;
    let created_user_id = sqlx::query("SELECT id FROM users WHERE username = $1 LIMIT 1")
        .bind(username)
        .fetch_one(&pool)
        .await
        .expect("could not fetch from database")
        .get::<Uuid, _>("id");

    // Assert
    res.assert_status(StatusCode::CREATED);
    assert_eq!(res.header("location"), format!("/users/{created_user_id}"));
    res.assert_json_contains(&json!({"id": created_user_id}));
}

#[sqlx::test]
async fn read(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user = seed_user(&pool).await;

    // Act
    let res = server.get(format!("/users/{}", user.id).as_str()).await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!({
        "id": user.id,
        "first_name": user.first_name,
        "last_name": user.last_name,
        "username": user.username,
        "email": user.email
    }));
}

#[sqlx::test]
async fn list(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user_1 = seed_user(&pool).await;
    let user_2 = seed_user(&pool).await;

    // Act
    let res = server.get("/users").await;

    // Assert
    res.assert_status(StatusCode::OK);
    res.assert_json_contains(&json!([
    {
        "id": user_1.id,
        "first_name": user_1.first_name,
        "last_name": user_1.last_name,
        "username": user_1.username,
        "email": user_1.email
    },
    {
        "id": user_2.id,
        "first_name": user_2.first_name,
        "last_name": user_2.last_name,
        "username": user_2.username,
        "email": user_2.email
    }
    ]));
}

#[sqlx::test]
async fn update(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user = seed_user(&pool).await;
    let updated_user = DbUser {
        id: user.id,
        ..Faker.fake()
    };
    let update_body = json!({
        "first_name": updated_user.first_name,
        "last_name": updated_user.last_name,
        "username": updated_user.username,
        "email": updated_user.email,
    });

    // Act
    let res = server
        .put(format!("/users/{}", user.id).as_str())
        .json(&update_body)
        .await;
    let written_user = query_as::<_, DbUser>("SELECT * FROM users WHERE id = $1 LIMIT 1")
        .bind(updated_user.id)
        .fetch_one(&pool)
        .await
        .expect("could not read user from db");

    // Assert
    res.assert_status(StatusCode::OK);
    assert_eq!(written_user, updated_user);
    res.assert_json_contains(&json!({
        "id": updated_user.id,
        "first_name": updated_user.first_name,
        "last_name": updated_user.last_name,
        "username": updated_user.username,
        "email": updated_user.email
    }));
}

#[sqlx::test]
async fn delete(pool: PgPool) {
    // Arrange
    let server = common::test_server(&pool);
    let user = seed_user(&pool).await;
    assert!(
        query_as::<_, DbUser>("SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(user.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read user from db")
            .is_some()
    );

    // Act
    let res = server.delete(format!("/users/{}", user.id).as_str()).await;

    // Assert
    res.assert_status(StatusCode::NO_CONTENT);
    assert!(res.into_bytes().is_empty());
    assert!(
        query_as::<_, DbUser>("SELECT * FROM users WHERE id = $1 LIMIT 1")
            .bind(user.id)
            .fetch_optional(&pool)
            .await
            .expect("could not read user from db")
            .is_none()
    );
}
