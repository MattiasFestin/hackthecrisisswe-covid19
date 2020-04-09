use std::time::SystemTime;
use serde::Serialize;
use uuid::Uuid;
// use rocket_contrib::uuid::Uuid;
use rocket_contrib::json::Json;
use crate::diesel::*;
use crate::view_models::*;
use rocket::State;

use crate::geoencoding::*;

#[get("/transaction?<id>")]
pub fn getTransaction<'r>(id: rocket_contrib::uuid::Uuid, db: crate::db::DbConn, mut redis: crate::redis::RedisConn) -> rocket_contrib::json::Json<Option<VM_Transaction>> {
    let db = &*db;
    let mut redis = &mut *redis;

    let dbId = Uuid::from_bytes(id.as_bytes()).unwrap();

    let mut results: Vec<crate::db_models::Transaction> = crate::schema::transactions::table
        .find(dbId)
        .load::<crate::db_models::Transaction>(db)
        .expect("Error loading posts");

    if let Some(rec) = results.pop() {
        return rocket_contrib::json::Json(Some(mapToViewmodel(&rec, &db, &mut redis)));
    }
    
    return rocket_contrib::json::Json(None);
}

fn getConstraint(parent_id: uuid::Uuid, db: &diesel::PgConnection) -> Vec<VM_Constraint> {
    use crate::schema::transactions_constraints::dsl::*;

    let mut results: Vec<crate::db_models::Transaction_Constraint> = transactions_constraints
        .filter(transactions_id.eq(parent_id))
        .load::<crate::db_models::Transaction_Constraint>(db)
        .expect("Error loading posts");
    
    return results.iter()
        .map(|rec| {
            return VM_Constraint {
                id: rec.id,
                created: rec.created,
                modified: rec.modified,
                deleted: rec.deleted,
                row_version: rec.row_version,
            
                op: rec.op,
                unit: rec.unit.clone(),
                name: rec.name.clone(),
            
                value: rec.value,
            };
        })
        .collect();
}

fn mapToViewmodel<'r>(rec: &crate::db_models::Transaction, db: &'r diesel::PgConnection, redis: &'r mut r2d2_redis::redis::Connection) -> VM_Transaction {
    let mut someAdd: Option<crate::geoencoding::Address> = None;
    match getAddress(rec.lat, rec.lng, redis) {
        Ok(addr) => {
            someAdd = addr;
        }
        Err(err) => {
            // println!("ERR: {}", err);
        }
        _ => {
            // println!("unkown err");
        }
    }

    return VM_Transaction {
        id: rec.id,
        created: rec.created,
        modified: rec.modified,
        deleted: rec.deleted,
        row_version: rec.row_version,
    
        transaction_direction_id: rec.transaction_direction_id,
        transaction_type_id: rec.transaction_type_id,
    
        lat: rec.lat,
        lng: rec.lng,

        what: rec.what.clone(),
        r#where: someAdd,
        
        priority: rec.priority,
        
        constraints: getConstraint(rec.id, &db),
    };
}

#[get("/transactions")]
pub fn getTransactionList(db: crate::db::DbConn, mut redis: crate::redis::RedisConn) -> rocket_contrib::json::Json<Vec<VM_Transaction>> {
    use crate::schema::transactions::dsl::*;

    let db = &*db;
    let mut redis = &mut *redis;
    
    let results: Vec<crate::db_models::Transaction> = transactions
        .load::<crate::db_models::Transaction>(db)
        .expect("Error loading posts");


    return rocket_contrib::json::Json(results.iter()
        .map(|rec| {
            return mapToViewmodel(rec, db, &mut redis);
        })
        .collect()
    );
}

#[put("/transaction", format = "json", data = "<data>")]
pub fn insertTransaction(data: Json<VM_Insert_Transaction>, db: crate::db::DbConn, mut redis: crate::redis::RedisConn) -> rocket_contrib::json::Json<VM_Transaction> {
    use crate::schema::transactions::dsl::*;

    let db = &*db;
    let mut redis = &mut *redis;

    let new_transaction = crate::db_models::Transaction {
        id: uuid::Uuid::new_v4(),
        created: SystemTime::now(),
        modified: SystemTime::now(),
        deleted: None,
        row_version: 1,

        transaction_direction_id: data.transaction_direction_id,
        transaction_type_id: data.transaction_type_id,

        lat: data.lat,
        lng: data.lng,

        what: data.what.clone(),
        priority: data.priority,
    };

    diesel::insert_into(transactions)
        .values(&new_transaction)
        .execute(db)
        .expect("Error saving new post");

    return rocket_contrib::json::Json(mapToViewmodel(&new_transaction, db, &mut redis));
}

#[post("/transaction", format = "json", data = "<data>")]
pub fn updateTransaction(data: Json<VM_Update_Transaction>, db: crate::db::DbConn, mut redis: crate::redis::RedisConn) -> () {
    use crate::schema::transactions::dsl::*;

    let db = &*db;
    let mut redis = &mut *redis;

    let new_transaction = crate::db_models::Transaction {
        // id: uuid::Uuid::new_v4(),
        id: data.id,
        created: data.created,
        modified: SystemTime::now(),
        deleted: None,
        row_version: data.row_version + 1,

        transaction_direction_id: data.transaction_direction_id,
        transaction_type_id: data.transaction_type_id,

        lat: data.lat,
        lng: data.lng,

        what: data.what.clone(),
        priority: data.priority,
    };

    diesel::update(transactions.find(id))
        .set(&new_transaction)
        .execute(db)
        .expect("Error saving new post");
}

#[delete("/transaction?<id>")]
pub fn deleteTransaction(id: rocket_contrib::uuid::Uuid, db: crate::db::DbConn, mut redis: crate::redis::RedisConn) -> () {
    use crate::schema::transactions::dsl::*;

    let db = &*db;
    let mut redis = &mut *redis;

    diesel::delete(transactions.find(id))
        .execute(db)
        .expect("Error saving new post");
}