use anyhow::Context;
use axum::serve;
use clap::Parser;
use fuel_logger_rs::{
    build_router,
    configuration::{read_config, Configuration, LogFormat},
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(clap::Parser, Debug)]
#[command(version)]
/// Vehicle Logger API
///
/// Run a webserver hosting a vehicle log API.
struct Cli {
    /// Print a sample configuration to stdout and then exit
    #[arg(long)]
    init_config: bool,
}

async fn run(config: Configuration) -> anyhow::Result<()> {
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
    tracing::info!("serving application on: {addr}");
    serve(listener, app)
        .await
        .context("failed to serve axum app")?;
    Ok(())
}

#[tracing::instrument]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load config
    let config = read_config().context("failed to load configuration")?;

    // Set tracing filter for log level
    let filter = EnvFilter::default()
        .add_directive(config.log.level.to_string().parse()?)
        .add_directive("sqlx=info".parse()?);

    // Set tracing layers for log format
    // Pattern from from https://github.com/tokio-rs/tracing/blob/master/examples/examples/toggle-subscribers.rs
    let (pretty, compact, full) = match config.log.format {
        LogFormat::Pretty => (Some(tracing_subscriber::fmt::layer().pretty()), None, None),
        LogFormat::Compact => (None, Some(tracing_subscriber::fmt::layer().compact()), None),
        LogFormat::Full => (None, None, Some(tracing_subscriber::fmt::layer())),
    };

    // Initialize tracing subscriber registry with filter and format layers
    tracing_subscriber::registry()
        .with(filter)
        .with(pretty)
        .with(compact)
        .with(full)
        .try_init()
        .context("registering tracing subscriber")?;

    // Handle CLI command
    let cli_args = Cli::parse();
    if cli_args.init_config {
        print!(include_str!("../dev/example-config.yml"));
        Ok(())
    } else {
        tracing::debug!("running application");
        run(config).await
    }
}
