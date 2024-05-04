use axum::{extract::State, Json};
use tracing::debug;

use crate::{database::AnimalOwnerController as aoc, ModelManager, Result};
use common::{AnimalRequest, AnimalResponse, Animals, Owner, OwnerRequest, OwnerResponse};

// region: ---- REST Handlers

async fn create_owner(
    State(mm): State<ModelManager>,
    Json(owner): Json<OwnerRequest>,
) -> Result<Json<OwnerResponse>> {
    debug!("{:<12} - CREATING ANIMAL", "HANDLER");

    Ok(Json(aoc::create_owner(&mm, owner).await.unwrap()))
}

// endregion: ---- REST Handlers
