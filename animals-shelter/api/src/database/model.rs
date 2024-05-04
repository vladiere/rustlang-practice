use common::{AnimalRequest, AnimalResponse, Animals, Owner, OwnerRequest, OwnerResponse};
use sqlx::Row;

use super::{ModelManager, Result};

pub struct AnimalOwnerController;

impl AnimalOwnerController {
    pub async fn create_owner(mm: &ModelManager, owner: OwnerRequest) -> Result<OwnerResponse> {
        let db = mm.db();
        let query = "INSERT INTO owner_table (name) VALUES ($1) RETURNING id, name";

        Ok(sqlx::query_as::<_, OwnerResponse>(query)
            .bind(owner.name)
            .fetch_one(db)
            .await?)
    }

    pub async fn fetch_one_owner(mm: &ModelManager, owner_id: i32) -> Result<OwnerResponse> {
        let db = mm.db();
        let query = "SELECT id, name FROM owner_table WHERE id = $1";

        Ok(sqlx::query_as::<_, OwnerResponse>(query)
            .bind(owner_id)
            .fetch_one(db)
            .await?)
    }

    pub async fn fetch_all_owner(mm: &ModelManager) -> Result<Vec<OwnerResponse>> {
        let db = mm.db();
        let query = "SELECT id, name FROM owner_table";

        Ok(sqlx::query_as::<_, OwnerResponse>(query)
            .fetch_all(db)
            .await?)
    }

    pub async fn create_animal(
        mm: &ModelManager,
        owner_id: i32,
        animal: AnimalRequest,
    ) -> Result<AnimalResponse> {
        let db = mm.db();
        let query = "INSERT INTO animal_table (owner_id,breed,color,name) VALUES ($1,$2,$3,$4) RETURNING id, breed, color, name";

        Ok(sqlx::query_as::<_, AnimalResponse>(query)
            .bind(owner_id)
            .bind(animal.breed)
            .bind(animal.color)
            .bind(animal.name)
            .fetch_one(db)
            .await?)
    }

    pub async fn fetch_animals(mm: &ModelManager, owner_id: i32) -> Result<Vec<AnimalResponse>> {
        let db = mm.db();
        let query = "SELECT id, breed, color, name FROM animal_table WHERE owner_id = $1";

        Ok(sqlx::query_as::<_, AnimalResponse>(query)
            .bind(owner_id)
            .fetch_all(db)
            .await?)
    }
}
