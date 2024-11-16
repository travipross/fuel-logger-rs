mod controllers;
mod error;
mod routes;
mod types;

use std::{env, time::Duration};

use anyhow::Context;
use axum::{routing::get, Router};
use routes::{log_records, users, vehicles};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .context("can't connect to database")?;

    let state = AppState { db: pool };

    // Build main app router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/vehicles", vehicles::build_router())
        .nest("/logs", log_records::build_router())
        .nest("/users", users::build_router())
        .with_state(state);

    let port = env::var("PORT").unwrap_or("3000".to_owned());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .context("failed to create TCP listener")?;
    println!("Running on: localhost:{port}");
    axum::serve(listener, app)
        .await
        .context("failed to serve axum app")?;
    Ok(())
}
