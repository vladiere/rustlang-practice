mod config;
mod database;
mod error;
mod routes;

use axum::{middleware, Router};
pub use config::config; // to use in the crate.

use database::db_model::ModelController;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app = Router::new().layer(ModelController::new().clone());
}
