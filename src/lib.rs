pub mod configuration;
pub mod controllers;
pub mod error;
pub mod models;
pub mod routes;
pub mod types;
pub mod utils;

use axum::Router;
use routes::{log_records, users, vehicles};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

pub fn build_router(pool: &PgPool) -> Router {
    let state = AppState { db: pool.clone() };
    Router::new()
        .nest("/users", users::build_router())
        .nest("/vehicles", vehicles::build_router())
        .nest("/log_records", log_records::build_router())
        .with_state(state)
}
