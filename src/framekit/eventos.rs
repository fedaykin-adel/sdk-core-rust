use crate::entities::common::Date;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct EventoInput {
    pub shaayud_id: String,
    pub fingerprint: JsonValue,
    pub ip: String,
    pub user_agent: String,
    pub timestamp: Date,
    pub method: String,
}
