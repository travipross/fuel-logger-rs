use anyhow::Context;
use axum::serve;
use clap::Parser;
use fuel_logger_rs::{build_router, configuration::read_config};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[derive(clap::Parser, Debug)]
#[command(version)]
/// Vehicle Logger API
///
/// Run a webserver hosting a vehicle log API.
struct Cli {
    /// Print a sample configuration to stdout
    #[arg(long)]
    init_config: bool,
}

async fn run() -> anyhow::Result<()> {
    let config = read_config().context("failed to load configuration")?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&config.database.url)
        .await
        .context("can't connect to database")?;

    // Build main app router
    let app = build_router(&pool);

    // let port = env::var("PORT").unwrap_or("3000".to_owned());
    let addr = format!("{}:{}", config.server.host, config.server.port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("failed to create TCP listener")?;
    println!("Running on: {addr}");
    serve(listener, app)
        .await
        .context("failed to serve axum app")?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli_args = Cli::parse();
    if cli_args.init_config {
        print!(include_str!("../dev/example-config.yml"));
        Ok(())
    } else {
        run().await
    }
}
