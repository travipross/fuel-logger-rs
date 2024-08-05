mod controllers;
mod models;
mod routes;

use std::{env, time::Duration};

use axum::{routing::get, Router};
use routes::{log_records, vehicles};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

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
