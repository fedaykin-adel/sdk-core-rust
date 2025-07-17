use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub user_id: String,
    pub typing_speed: f32,
    pub shaayud_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub score: f32,
}
