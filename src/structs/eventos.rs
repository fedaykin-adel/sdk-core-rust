use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoPayload {
    pub country: Option<String>,
    pub region: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct EventoInput {
    pub shaayud_id: String,
    pub fingerprint: JsonValue,
    pub ip: String,
    pub user_agent: String,
    pub header: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub path: String,
    pub session_id: Option<String>,
    pub event_id: Option<String>,
    pub event_type: Option<String>,

    pub geo: Option<GeoPayload>,

    pub front_url: Option<String>,
    pub front_path: Option<String>,
    pub front_referrer: Option<String>,

    pub backend_path: Option<String>,
    pub backend_method: Option<String>,
    pub backend_host: Option<String>,
}
