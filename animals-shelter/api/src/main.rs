#![allow(unused)]

mod config;
mod database;
mod error;
mod routes;

use axum::{middleware, routing::get, Router};
pub use config::config;
use tracing::info; // to use in the crate.

pub use self::error::{Error, Result};
use database::{dev_utils, ModelManager};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    dev_utils::init_dev_db().await;

    // let mm = ModelManager::new().await.unwrap();

    // let app = Router::new().with_state(mm.clone());
    let router = Router::new().route("/", get(|| async { "Hello, World!" }));

    let host_addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(host_addr).await.unwrap();

    info!("Server listening on http://{}", host_addr);
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
