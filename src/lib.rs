mod eventos;
mod structs_custom;

pub use crate::eventos::EventoInput;
pub use structs_custom::{DataRequest, UserData, VerificationResult};

use crate::eventos::ActiveModel;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;

pub fn verify_user(data: UserData) -> VerificationResult {
    let score = data.typing_speed.min(100.0);

    let valid = score >= 50.0;
    VerificationResult { valid, score }
}
pub fn recive_data(data: DataRequest) {
    // logica aqui
    println!("[Shaayud] Data received: {:?}", data);
}
pub async fn handle_ingest(data: EventoInput, db: &DatabaseConnection) -> Result<(), DbErr> {
    let model: ActiveModel = ActiveModel {
        fingerprint: Set(data.fingerprint),
        shaayud_id: Set(data.shaayud_id),
        ..Default::default()
    };
    model.insert(db).await?;
    Ok(())
}

// ip: Set(data.ip),
// user_agent: Set(data.user_agent),
// timezone: Set(data.timezone),
// body: Set(data.body),
// header: Set(data.header),
