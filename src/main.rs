mod models;
mod routes;

use fake::{Fake, Faker};
use models::{LogRecord, Person, Vehicle, VehicleMake, VehicleModel, VehicleYear};

use axum::{routing::get, Router};
use routes::{get_logs, get_vehicle, get_vehicles};
#[tokio::main]
async fn main() {
    let person = Faker.fake::<Person>();

    let mut civic = Vehicle::new(
        VehicleMake("Honda".to_owned()),
        VehicleModel("Civic".to_owned()),
        VehicleYear(2020),
        person,
        models::OdometerUnit::Metric,
    );

    let record1 = Faker.fake::<LogRecord>();
    let record2 = Faker.fake::<LogRecord>();

    civic.add_record(record1);
    civic.add_record(record2);

    dbg!(&civic);

    println!("Civic JSON: {}", serde_json::to_string(&civic).unwrap());
    println!("Current odometer: {}", civic.get_current_odo());

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/logs/:vehicle_id", get(get_logs))
        .route("/vehicle/:vehicle_id", get(get_vehicle))
        .route("/vehicles", get(get_vehicles));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on: localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
