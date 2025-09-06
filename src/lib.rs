mod framekit;
pub mod structs;

pub use framekit::fingerprint::{ParsedDevice, extract_device_info};
use neo4rs::Graph;
use neo4rs::query;
use serde_json::json;
use std::sync::Arc;
pub use structs::eventos::EventoInput;

use crate::framekit::event::ParsedEvent;
use crate::framekit::event::extract_event_info;
use crate::framekit::geo::ParsedGeo;
use crate::framekit::geo::extract_geo_info;
use crate::framekit::identity::ParsedIdentity;
use crate::framekit::identity::extract_identity_info;
use crate::framekit::network::ParsedNetwork;
use crate::framekit::network::extract_network_info;
use crate::framekit::session::ParsedSession;
use crate::framekit::session::extract_session_info;

// pub fn verify_user(data: UserData) -> VerificationResult {
//     let score = data.typing_speed.min(100.0);

//     let valid = score >= 50.0;
//     VerificationResult { valid, score }
// }

pub async fn handle_ingest(data: EventoInput, graph: &Arc<Graph>) -> Result<(), neo4rs::Error> {
    match serde_json::to_string_pretty(&data) {
        Ok(j) => tracing::debug!("📨 EventoInput:\n{}", j),
        Err(e) => tracing::warn!("⚠️  Falha ao serializar EventoInput: {e}"),
    }

    let dev: ParsedDevice = extract_device_info(&data.fingerprint);
    let device_id = dev.id;
    let os = dev.os.unwrap_or_default();
    let browser = dev.browser.unwrap_or_default();
    let device_type = dev.device_type.unwrap_or_else(|| "unknown".to_string());

    let idy: ParsedIdentity = extract_identity_info(&data.shaayud_id);
    let identity_id = idy.id;

    let sess: ParsedSession =
        extract_session_info(&data.header, &data.session_id, &device_id, data.timestamp);

    let net: ParsedNetwork = extract_network_info(&data.ip, &data.user_agent);

    let evt: ParsedEvent = extract_event_info(
        &data.event_id,
        &data.event_type,
        &data.method,
        &data.path,
        &identity_id,
        &sess.id,
        data.timestamp,
    );

    let geo: ParsedGeo = extract_geo_info(&data.geo, &data.header);

    let params = json!({
        "identityId": identity_id,
        "deviceId": device_id,
        "os": os,
        "browser": browser,
        "deviceType": device_type,
        "ts_ms": evt.ts_ms,
        "sessionId": sess.id,

        "ip": net.ip,
        "uaRaw": net.ua_raw,
        "eventId": evt.id,
        "eventType": evt.r#type,

        "geoKey": if geo.key.is_empty() { None } else { Some(geo.key) },
        "geoCountry": geo.country,
        "geoRegion": geo.region,
        "geoCity": geo.city,
        "geoTimezone": geo.timezone,
        "geoLat": geo.latitude,
        "geoLng": geo.longitude,

        "front_url": data.front_url,
        "front_path": data.front_path,
        "front_referrer": data.front_referrer,

        "backend_path": data.backend_path,
        "backend_method": data.backend_method,
        "backend_host": data.backend_host,
    });
    tracing::debug!("🧪 Params p/ Neo4j: {params}");

    const UPSERT: &str = r#"
    WITH datetime({epochMillis: $ts_ms}) AS ts
    MERGE (i:Identity {id: $identityId})
    ON CREATE SET i.createdAt = ts
    WITH i, ts
    MERGE (d:Device {id: $deviceId})
    ON CREATE SET d.first_seen = ts
    SET d.os = $os, d.browser = $browser, d.device_type = $deviceType, d.last_seen = ts
    MERGE (i)-[:USES]->(d)
    WITH i, d, ts
    MERGE (s:Session {id: $sessionId})
    ON CREATE SET s.startedAt = ts
    SET s.ip = coalesce($ip, s.ip),
        s.ua = coalesce($uaRaw, s.ua)
    MERGE (d)-[:OPENED]->(s)
    WITH s, ts, $ip AS pip
    MERGE (e:Event {id: $eventId})
    SET e.type = $eventType,
        e.ts = ts,
        e.front_url = coalesce($front_url, e.front_url),
        e.front_path = coalesce($front_path, e.front_path),
        e.front_referrer = coalesce($front_referrer, e.front_referrer),
        e.backend_path = coalesce($backend_path, e.backend_path),
        e.backend_method = coalesce($backend_method, e.backend_method),
        e.backend_host = coalesce($backend_host, e.backend_host)
    MERGE (s)-[:EMITTED]->(e)
    WITH s, pip, $geoKey AS gk
    FOREACH (_ IN CASE WHEN pip IS NULL OR pip = '' THEN [] ELSE [1] END |
    MERGE (ip:IP {addr: pip})
    MERGE (s)-[:FROM_IP]->(ip)
    )
    FOREACH (_ IN CASE WHEN gk IS NULL OR gk = '' THEN [] ELSE [1] END |
    MERGE (g:Geo {key: gk})
        SET g.country  = coalesce($geoCountry, g.country),
            g.region   = coalesce($geoRegion, g.region),
            g.city     = coalesce($geoCity, g.city),
            g.timezone = coalesce($geoTimezone, g.timezone),
            g.latitude = coalesce($geoLat, g.latitude),
            g.longitude= coalesce($geoLng, g.longitude)
    MERGE (s)-[:LOCATED_IN]->(g)
    FOREACH (__ IN CASE WHEN pip IS NULL OR pip = '' THEN [] ELSE [1] END |
        MERGE (ip2:IP {addr: pip})
        MERGE (ip2)-[:LOCATED_IN]->(g)
    )
    )
    "#;
    let mut tx = graph.start_txn().await.map_err(|e| {
        tracing::error!("🚫 start_tx falhou: {e}");
        e
    })?;
    let mut q = query(UPSERT);
    for (k, v) in params.as_object().unwrap() {
        match v {
            serde_json::Value::String(s) => {
                q = q.param(k, s.as_str());
            }
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    q = q.param(k, i);
                } else if let Some(f) = n.as_f64() {
                    q = q.param(k, f);
                }
            }
            serde_json::Value::Bool(b) => {
                q = q.param(k, *b);
            }
            serde_json::Value::Null => {
                q = q.param::<Option<&str>>(k, None);
            }
            _ => {
                // For unsupported types, skip or handle as needed
                tracing::warn!("Unsupported param type for key: {}", k);
            }
        }
    }
    if let Err(e) = tx.run(q).await {
        tracing::error!("💥 Erro ao executar Cypher: {e}");
        return Err(e);
    }
    if let Err(e) = tx.commit().await {
        tracing::error!("💥 Erro ao dar commit na transação: {e}");
        return Err(e);
    }
    tracing::info!("✅ ingest concluído");

    Ok(())
}
