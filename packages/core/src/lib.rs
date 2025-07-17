mod entities;
mod framekit;

mod structs_custom;
use crate::entities::events::ActiveModel;
pub use framekit::eventos::EventoInput;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
pub use structs_custom::{UserData, VerificationResult};

pub fn verify_user(data: UserData) -> VerificationResult {
    let score = data.typing_speed.min(100.0);

    let valid = score >= 50.0;
    VerificationResult { valid, score }
}

pub async fn handle_ingest(data: EventoInput, db: &DatabaseConnection) -> Result<(), DbErr> {
    let model: ActiveModel = ActiveModel {
        fingerprint: Set(data.fingerprint),
        shaayud_id: Set(data.shaayud_id),
        user_agent: Set(data.user_agent),
        method: Set(data.method),
        ip: Set(data.ip),
        timestamp: Set(data.timestamp),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}
