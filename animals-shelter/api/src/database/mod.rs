// region: ---- Modules

use self::{
    error::Result,
    store::{new_db_pool, Db},
};

pub mod dev_utils;
pub mod error;
mod model;
pub mod store;

pub use model::*;

// endregion: ---- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db,
}

impl ModelManager {
    // ModelManager Constructor
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;

        Ok(ModelManager { db })
    }

    // Returns the sqlx db pool reference.
    /// (Only for the database layer)
    pub(in crate::database) fn db(&self) -> &Db {
        &self.db
    }
}
