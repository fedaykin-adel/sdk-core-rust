use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Debug, Serialize, Deserialize)]
// pub body: TBody,
// pub header: THeader,
// pub ip: String,
// pub user_agent: String,
// pub timezone: Date,
pub struct DataRequest<TFingerPrint = Value> {
    pub shaayud_id: String,
    pub fingerprint: TFingerPrint,
    
}

// user_id: 'anonymous',
//     shaayudId:shaayudId,
//     // typing_speed: Math.random() * 100,
//     fingerprint: result