use std::env;

use ::tracing::{debug, info};
use clap::Parser;
use miette::Result;

mod cli;
mod database;
mod error;
mod http;
mod tracing;

pub use error::Error;

fn init_tracing(opts: &cli::Opts) -> Result<(), Error> {
    #[cfg(feature = "tracing")]
    tracing::init(&opts.tracing_opts)?;
    #[cfg(not(feature = "tracing"))]
    tracing::init()?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Override RUST_LOG with a default setting if it's not set by the user
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "redirekt=trace,tower_http=debug");
    }

    let opts = cli::Opts::parse();
    init_tracing(&opts)?;

    let version = env!("CARGO_PKG_VERSION");
    info!(version, "Starting redirekt");

    debug!("connecting to database");
    let pool = database::open(&opts.database_url).await?;
    debug!(kind = ?pool.any_kind(), "connected to database");

    info!("starting http server");
    let handle = axum_server::Handle::new();

    http::start_server(&opts.http_opts, handle.clone(), pool.clone()).await?;

    println!("Hello, world! {:?}", opts);

    Ok(())
}
