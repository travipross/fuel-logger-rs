use anyhow::Context;
use axum::serve;
use fuel_logger_rs::build_router;
use sqlx::postgres::PgPoolOptions;
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .context("can't connect to database")?;

    // Build main app router
    let app = build_router(&pool);

    let port = env::var("PORT").unwrap_or("3000".to_owned());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .context("failed to create TCP listener")?;
    println!("Running on: localhost:{port}");
    serve(listener, app)
        .await
        .context("failed to serve axum app")?;
    Ok(())
}
