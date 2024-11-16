mod controllers;
mod error;
mod models;
mod routes;

use std::{env, time::Duration};

use axum::{routing::get, Router};
use fake::{Fake, Faker};
use models::User;
use routes::{log_records, vehicles};
use sqlx::{postgres::PgPoolOptions, query, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

pub const DEFAULT_USER_ID: &str = "50c5ab2e-4c29-4583-a698-5902b861b628";

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let mut fake_user = Faker.fake::<User>();
    fake_user.id = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
    query!("INSERT INTO users (id, first_name, last_name, username, email) VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING", fake_user.id, fake_user.first_name, fake_user.last_name, fake_user.username, fake_user.email).execute(&pool).await.unwrap();

    let state = AppState { db: pool };

    // Build main app router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/vehicles", vehicles::build_router())
        .nest("/logs", log_records::build_router())
        .with_state(state);

    let port = env::var("PORT").unwrap_or("3000".to_owned());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Running on: localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}
