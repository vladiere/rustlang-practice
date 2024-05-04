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
pub struct Posts {
    pub id: i32,
    pub owner_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct PostsRequest {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, FromRow)]
pub struct PostsResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
}

impl PostsResponse {
    pub fn of(post: Posts) -> PostsResponse {
        PostsResponse {
            id: post.id,
            title: post.title,
            content: post.content,
        }
    }
}
