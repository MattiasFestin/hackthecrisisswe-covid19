use std::time::SystemTime;

#[derive(Queryable)]
pub struct Users {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    username: String,
    hash: String,
    roles: Vec<String>
}

#[derive(Queryable)]
pub struct TransactionTypes {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    name: String
}

#[derive(Queryable)]
pub struct TransactionDir {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    name: String
}

pub enum TransactionDirEnum {
    Request,
    Offer
}

pub enum TransactionTypesEnum {
    Work,
    Material,
    Transport,
    Production
}

pub enum ConstraintTypeEnum {
    Boolean,
    List,
    Numeric
}

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

#[derive(Queryable)]
pub struct TransactionConstraint {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    name: String,
    unit: Unit,
    constraint_type: ConstraintTypeEnum
}

#[derive(Queryable)]
pub struct Unit {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    name: String
}

#[derive(Queryable)]
pub struct Transaction {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    coordinate: geo::Point<f64>,

    transaction_dir: TransactionDirEnum,
    transaction_type: TransactionTypesEnum,
    constraints: Vec<uuid::Uuid>,
}

// #[derive(Queryable)]
// pub struct TransactionOffer {
//     id: uuid::Uuid,
//     created: SystemTime,
//     modified: SystemTime,
//     deleted: bool,
//     row_version: i32,

//     transaction_dir: TransactionDirEnum,
//     transaction_type: TransactionTypesEnum,
//     constraints: Vec<uuid::Uuid>,
// }
