mod models;
mod routes;

use std::env;

use axum::{routing::get, Router};
use routes::{log_records, vehicles};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/vehicles", vehicles::build_router())
        .nest("/logs", log_records::build_router());

    let port = env::var("PORT").unwrap_or("3000".to_owned());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    println!("Running on: localhost:{port}");
    axum::serve(listener, app).await.unwrap();
}
