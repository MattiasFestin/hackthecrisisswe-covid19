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

#[derive(Queryable)]
pub struct TransactionConstraint {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    name: String,
    unit: Unit
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
pub struct TransactionRequest {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    transaction_dir: TransactionDirEnum,
    transaction_type: TransactionTypesEnum,
    constraints: Vec<uuid::Uuid>,
}

#[derive(Queryable)]
pub struct TransactionOffer {
    id: uuid::Uuid,
    created: SystemTime,
    modified: SystemTime,
    deleted: bool,
    row_version: i32,

    transaction_dir: TransactionDirEnum,
    transaction_type: TransactionTypesEnum,
    constraints: Vec<uuid::Uuid>,
}
