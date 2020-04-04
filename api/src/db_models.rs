use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize)]
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

#[derive(Queryable, Serialize, Deserialize)]
pub struct TransactionTypes {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: u32,

    pub name: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TransactionDir {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: u32,

    pub name: String
}

#[derive(Serialize, Deserialize)]
pub enum TransactionDirEnum {
    Request,
    Offer
}

#[derive(Serialize, Deserialize)]
pub enum TransactionTypesEnum {
    Work,
    Material,
    Transport,
    Production
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintTypeEnum {
    Boolean,
    List,
    Numeric
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintTypeOpEnum {
    bool_eq,

    list_all,
    list_some,
    
    numeric_eq,
    numeric_leq,
    numeric_geq,
    numeric_gt,
    numeric_lt
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct TransactionConstraint {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: u32,

    pub name: String,
    pub unit: Unit,
    pub constraint_type: ConstraintTypeEnum
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Unit {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: u32,

    pub name: String
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub transaction_direction_id: i64,
    pub transaction_type_id: i64,

    pub lat: f32,
    pub lng: f32,
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
