use crate::entities::common::Date;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct EventoInput {
    pub shaayud_id: String,
    pub fingerprint: JsonValue,
    pub ip: String,
    pub user_agent: String,
    pub header: JsonValue,
    pub timestamp: Date,
    pub method: String,
    pub path: String,
}
