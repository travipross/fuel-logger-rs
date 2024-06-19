mod models;
use models::{LogRecord, Person, Vehicle};

use axum::{routing::get, Router};
#[tokio::main]
async fn main() {
    let travis = Person {
        username: "travipross".to_owned(),
        email: "travi@pross.com".to_owned(),
        first_name: None,
        last_name: None,
    };

    let civic = Vehicle {
        make: "Honda".to_owned(),
        model: "Civic".to_owned(),
        year: 2020,
        owner: travis,
        odometer_unit: models::OdometerUnit::Metric,
        logs: vec![LogRecord {
            date: "today".to_string(),
            log_type: models::LogType::FuelUp { amount: 39.5 },
            odometer: 69420,
        }],
    };

    dbg!(civic);

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running on: localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
