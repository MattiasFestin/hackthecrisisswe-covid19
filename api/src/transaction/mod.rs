use std::time::SystemTime;
use serde::Serialize;
use uuid::Uuid;
// use rocket_contrib::uuid::Uuid;
use crate::diesel::*;
use crate::view_models::*;

#[get("/transaction?<id>")]
pub fn getTransaction(id: rocket_contrib::uuid::Uuid) -> rocket_contrib::json::Json<Option<VM_Transaction>> {
    
    let con = crate::apihelper::connect();    
    
    let dbId = Uuid::from_bytes(id.as_bytes()).unwrap();
    
    let mut results: Vec<crate::db_models::Transaction> = crate::schema::transactions::table
        .find(dbId)
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");

    return rocket_contrib::json::Json(
        if let Some(rec) = results.pop() {
            Some(VM_Transaction {
                id: rec.id,
                created: rec.created,
                modified: rec.modified,
                deleted: rec.deleted,
                row_version: rec.row_version,
            
                transaction_direction_id: rec.transaction_direction_id,
                transaction_type_id: rec.transaction_type_id,
            
                lat: rec.lat,
                lng: rec.lng,
                
                priority: rec.priority,

                what: rec.what.clone(),
                r#where: String::from("Hello, Rust!"),
            
                constraints: getConstraint(rec.id, &con.db),
            })
        } else {
            None
        }
    );
}

fn getConstraint(transaction_id: uuid::Uuid, db: &PgConnection) -> Vec<VM_Constraint> {
    let mut results: Vec<crate::db_models::Transaction_Constraint> = crate::schema::transactions_constraints::table
        .find(transaction_id)
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

#[get("/transactions")]
pub fn getTransactionList() -> rocket_contrib::json::Json<Vec<VM_Transaction>> {
    use crate::schema::transactions::dsl::*;

    let con = crate::apihelper::connect();    
    
    let results: Vec<crate::db_models::Transaction> = transactions.limit(1)
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");


    return rocket_contrib::json::Json(results.iter()
        .map(|rec| {
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
                r#where: String::from("Hello, Rust!"),
                
                priority: rec.priority,
                
                constraints: getConstraint(rec.id, &con.db),
            };
        })
        .collect()
    );
}