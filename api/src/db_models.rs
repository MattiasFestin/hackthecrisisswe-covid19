use std::time::SystemTime;
// use serde::{Serialize, Deserialize};
use crate::schema::*;
use crate::sql_types::PGPoint;

#[derive(Queryable)]
pub struct Users {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: u32,

    pub username: String,
    pub hash: String,
    pub roles: Vec<String>
}

// #[derive(Queryable, Serialize, Deserialize)]
// pub struct TransactionTypes {
//     pub id: uuid::Uuid,
//     pub created: SystemTime,
//     pub modified: SystemTime,
//     pub deleted: Option<SystemTime>,
//     pub row_version: u32,

//     pub name: String
// }

// #[derive(Queryable, Serialize, Deserialize)]
// pub struct TransactionDir {
//     pub id: uuid::Uuid,
//     pub created: SystemTime,
//     pub modified: SystemTime,
//     pub deleted: Option<SystemTime>,
//     pub row_version: u32,

//     pub name: String
// }


#[derive(Queryable)]
pub struct TransactionConstraint {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub transaction_id: uuid::Uuid,

    pub op: i64,
    pub name: String,
    pub unit: String,
    pub value: f32
}

// #[derive(Queryable)]
// pub struct Unit {
//     pub id: uuid::Uuid,
//     pub created: SystemTime,
//     pub modified: SystemTime,
//     pub deleted: Option<SystemTime>,
//     pub row_version: u32,

//     pub name: String
// }

#[derive(Queryable, Insertable, AsChangeset)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub transaction_type_id: i64,
    pub transaction_direction_id: i64,

    pub point: PGPoint,

    pub what: String,
    pub priority: i64,
}

// #[derive(Queryable)]
// pub struct TransactionOffer {
//     id: uuid::Uuid,
//     created: SystemTime,
//     modified: SystemTime,
//     deleted: bool,
//     row_version: u32,

//     transaction_dir: TransactionDirEnum,
//     transaction_type: TransactionTypesEnum,
//     constraints: Vec<uuid::Uuid>,
// }
