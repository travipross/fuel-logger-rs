use axum::{extract::Path, Json};
use fake::{Fake, Faker};

use crate::models::LogRecord;

pub async fn get_logs(Path(vehicle_id): Path<u32>) -> Json<Vec<LogRecord>> {
    println!("Getting logs for vehicle ID: {}", vehicle_id);
    let log_records = Faker.fake::<Vec<LogRecord>>();
    Json(log_records)
}
