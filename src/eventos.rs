use sea_orm::entity::prelude::*;
// use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "eventos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub shaayud_id: String,
    pub fingerprint: JsonValue,
}
// }
// pub ip: String,
// pub user_agent: String,
// pub fingerprint: JsonValue,
// pub timezone: String,
// pub body: JsonValue,
// pub header: JsonValue,

// #[derive(Debug, Deserialize)]
// pub struct EventoInput {
//     pub ip: String,
//     pub user_agent: String,
//     pub fingerprint: JsonValue,
//     pub timezone: String,
//     pub body: JsonValue,
//     pub header: JsonValue,
// }
#[derive(Debug, Deserialize)]
pub struct EventoInput {
    pub shaayud_id: String,
    pub fingerprint: JsonValue,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("no relations") // se não tiver nenhuma relação
    }
}

impl ActiveModelBehavior for ActiveModel {}

// CREATE TABLE eventos (
//     id int NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
//     create_time DATE,
//     ip TEXT NOT NULL,
//     user_agent TEXT NOT NULL,
//     fingerprint JSONB NOT NULL,
//     timezone TEXT NOT NULL,
//     body JSONB NOT NULL,
//     header JSONB NOT NULL
// );
