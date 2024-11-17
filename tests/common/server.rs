use axum_test::TestServer;
use sqlx::PgPool;

pub fn test_server(pool: &PgPool) -> TestServer {
    let app = fuel_logger_rs::build_router(pool);

    TestServer::new(app).expect("could not create test server")
}
