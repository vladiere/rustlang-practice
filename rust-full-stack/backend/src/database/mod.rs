use crate::error::{Error, Result};
// region: ---- Modules

pub mod _dev_utils;
pub mod db_model;
mod error;
pub mod routes;
pub use self::error::{Error, Result};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// endregion: ---- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
