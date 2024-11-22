pub mod configuration;
pub mod controllers;
pub mod error;
pub mod extractors;
pub mod models;
pub mod routes;
pub mod types;
pub mod utils;

use axum::Router;
use routes::{log_records, users, vehicles};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    db: PgPool,
}

#[tracing::instrument(name = "build_main_router", skip_all)]
pub fn build_router(pool: &PgPool) -> Router {
    tracing::debug!("building main router");
    let state = AppState { db: pool.clone() };
    Router::new()
        .nest("/users", users::build_router())
        .nest("/vehicles", vehicles::build_router())
        .nest("/log_records", log_records::build_router())
        .with_state(state)
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO)),
        )
}
