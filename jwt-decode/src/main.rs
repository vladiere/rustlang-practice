use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TokenClaims {
    id: i64,
    role_user: String,
    username: String,
    token_salt: String,
    exp: String,
}

fn main() {
    let token = "eyJhbGciOiJIUzI1NiJ9.eyJpZCI6MTAwMCwicm9sZV91c2VyIjoiVXNlciIsInVzZXJuYW1lIjoia3VsYXMuYWRtaW4iLCJ0b2tlbl9zYWx0IjoiZmE3MGZiZWMtNjZlYi00NTExLWEyYTMtNTNlOTU1MzBjYjRhIn0.ZThsoz4-3l_6Pu-pwTc7tG89h3LBIWUdh65k6KQPEME".to_string();
    let key = "OWL-project-2024";
    let extracted = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(key.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    println!("{extracted:?}");
}
