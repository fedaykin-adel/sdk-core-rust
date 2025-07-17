use crate::entities::common::Date;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "verifications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub user_id: String,
    pub device_id: String,
    pub ip: String,
    pub timestamp: Date,
    pub result: String, // "approved", "suspicious", "rejected"
    pub score: f32,
    pub reason: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("no relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}
