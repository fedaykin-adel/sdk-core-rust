mod entities;
mod framekit;
mod structs;
use crate::entities::events::ActiveModel as EventModel;
pub use framekit::fingerprint::{ParsedDevice, extract_device_info};
use neo4rs::Graph;
use neo4rs::query;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use std::sync::Arc;
pub use structs::eventos::EventoInput;

// pub fn verify_user(data: UserData) -> VerificationResult {
//     let score = data.typing_speed.min(100.0);

//     let valid = score >= 50.0;
//     VerificationResult { valid, score }
// }

pub async fn handle_ingest(
    data: EventoInput,
    db: &DatabaseConnection,
    graph: &Arc<Graph>,
) -> Result<(), DbErr> {
    // tracing::debug!("🔍 JSON recebido: {data}");

    let data_device: ParsedDevice = extract_device_info(&data.fingerprint);

    let id = data_device.id;
    let os = data_device.os.unwrap_or_default();
    let browser = data_device.browser.unwrap_or_default();
    let device_type = data_device
        .device_type
        .unwrap_or_else(|| "unknown".to_string());
    let timestamp = data.timestamp.to_string();

    let insert_device = query(
        "MERGE (d:Device {id: $id}) 
        SET d.os = $os, d.browser = $browser, d.device_type = $device_type, d.last_seen = $timestamp"
    )
    .param("id", id)
    .param("os", os)
    .param("browser", browser)
    .param("device_type", device_type)
    .param("timestamp", timestamp);

    graph
        .run(insert_device)
        .await
        .map_err(|e| DbErr::Custom(e.to_string()))?;

    // let model: EventModel = EventModel {
    //     fingerprint: Set(data.fingerprint),
    //     shaayud_id: Set(data.shaayud_id),
    //     user_agent: Set(data.user_agent),
    //     method: Set(data.method),
    //     ip: Set(data.ip),
    //     timestamp: Set(data.timestamp),
    //     ..Default::default()
    // };
    // model.insert(db).await?;
    Ok(())
}
