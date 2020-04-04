use std::time::SystemTime;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize)]
pub struct VM_Transaction {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub transaction_direction_id: i64,
    pub transaction_type_id: i64,

    pub priority: i64,

    pub what: String,
    pub r#where: String,

    pub lat: f32,
    pub lng: f32,

    pub constraints: Vec<VM_Constraint>,
}

#[derive(Serialize, Deserialize)]
pub struct VM_Constraint {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub name: String,
    pub unit: String,
    pub op: i64,
    pub value: f32
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