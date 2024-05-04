use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthResult {
    pub message: &'static str,
    pub status: u32,
}
