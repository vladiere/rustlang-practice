use super::Db;
use common::{Owner, OwnerResponse, OwnerResponse, PostResponse, PostsRequest};
use sqlx::Row;

#[derive(Clone)]
pub struct ModelController {
    pub db: Db,
}

// ---- Contructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        let db = super::new_db_pool();

        Ok(Self { db })
    }
}

impl ModelController {
    pub async fn create_owner(&self, owner: OwnerRequest) -> Result<Owner> {
        let query = "INSERT INTO owner_table (name) VALUES ($1) RETURNING id, name";

        let res = sqlx::query_as::<_, OwnerResponse>(query)
            .bind(owner.name)
            .fetch_one(&self.db)
            .await;

        Ok(res)
    }

    pub async fn fetch_one_owner(&self, owner_id: i32) -> Result<OwnerResponse> {
        let query = "SELECT id, name FROM owner_table WHERE id = $1";

        let res = sqlx::query_as::<_, OwnerResponse>(query)
            .bind(owner_id)
            .fetch_one(&self.db)
            .await;

        Ok(res)
    }

    pub async fn fetch_all_owner(&self) -> Result<Vec<OwnerResponse>> {
        let query = "SELECT id, name FROM owner_table";

        let res = sqlx::query_as::<_, OwnerResponse>(query)
            .fetch_all(&self.db)
            .await;

        Ok(row_to_owners(res))
    }

    pub async fn create_post(
        &self,
        owner_id: i32,
        create_post: PostsRequest,
    ) -> Result<PostResponse> {
        let query = "INSERT INTO post_table (owner_id,title,content) VALUES ($1,$2,$3) RETURNING id, title, content";

        let res = sqlx::query_as::<_, PostResponse>(query)
            .bind(owner_id)
            .bind(create_post.title)
            .bind(create_post.content)
            .fetch_one(&self.db)
            .await;

        Ok(res)
    }

    pub async fn fetch_post(&self, owner_id: i32) -> Result<Vec<PostResponse>> {
        let query = "SELECT id, title, content FROM post_table WHERE owner_id = $1";

        let res = sqlx::query_as::<_, PostResponse>(query)
            .bind(owner_id)
            .fetch_all(&self.db)
            .await;

        Ok(row_to_posts(res))
    }
}

fn row_to_owners(row: &Row) -> OwnerResponse {
    let id: i32 = row.get(0);
    let name: String = row.get(1);

    OwnerResponse { id, name }
}

fn row_to_posts(row: &Row) -> PostResponse {
    let id: i32 = row.get(0);
    let title: String = row.get(1);
    let content: String = row.get(2);

    PostResponse { id, title, content }
}
