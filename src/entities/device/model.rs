use crate::entities::common::Date;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "devices")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // pode ser hash do fingerprint
    pub os: String,
    pub browser: String,
    pub device_type: String, // ex: mobile, desktop
    pub first_seen: Date,
    pub last_seen: Date,
    pub confidence_score: f32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("no relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}
