use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Owner {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct OwnerRequest {
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, FromRow)]
pub struct OwnerResponse {
    pub id: i32,
    pub name: String,
}

impl OwnerResponse {
    pub fn of(owner: Owner) -> OwnerResponse {
        OwnerResponse {
            id: owner.id,
            name: owner.name,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct Animals {
    pub id: i32,
    pub owner_id: i32,
    pub breed: String,
    pub color: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct AnimalRequest {
    pub breed: String,
    pub color: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, FromRow)]
pub struct AnimalResponse {
    pub id: i32,
    pub breed: String,
    pub color: String,
    pub name: String,
}

impl AnimalResponse {
    pub fn of(animal: Animals) -> AnimalResponse {
        AnimalResponse {
            id: animal.id,
            breed: animal.breed,
            color: animal.color,
            name: animal.name,
        }
    }
}
