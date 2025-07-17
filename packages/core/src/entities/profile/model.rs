use crate::entities::common::Date;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "user_profiles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String, // shaayud_id
    pub last_ip: String,
    pub last_device_id: String,
    pub last_active: Date,
    pub behavior_score: f32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("no relations")
    }
}

impl ActiveModelBehavior for ActiveModel {}
