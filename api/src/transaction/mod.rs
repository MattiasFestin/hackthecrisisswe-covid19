use std::time::SystemTime;
use serde::Serialize;
use uuid::Uuid;
// use rocket_contrib::uuid::Uuid;
use crate::diesel::*;
use crate::view_models::*;

use crate::geoencoding::*;

#[get("/transaction?<id>")]
pub fn getTransaction(id: rocket_contrib::uuid::Uuid) -> rocket_contrib::json::Json<Option<VM_Transaction>> {
    let con = crate::apihelper::connect();    
    
    let dbId = Uuid::from_bytes(id.as_bytes()).unwrap();
    
    let mut results: Vec<crate::db_models::Transaction> = crate::schema::transactions::table
        .find(dbId)
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");

    if let Some(rec) = results.pop() {
        let mut someAdd: Option<crate::geoencoding::Address> = None;
        match getAddress(rec.lat, rec.lng) {
            Ok(addr) => {
                println!("OK: {:?}", addr);
                someAdd = addr;
            }
            Err(err) => {
                println!("ERR: {}", err);
            }
            _ => {
                println!("unkown err");
            }
        }

        return rocket_contrib::json::Json(Some(VM_Transaction {
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
            r#where: someAdd,//String::from("Hello, Rust!"),
        
            constraints: getConstraint(rec.id, &con.db),
        }));
    }
    
    return rocket_contrib::json::Json(None);
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
    
    let results: Vec<crate::db_models::Transaction> = transactions
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");


    return rocket_contrib::json::Json(results.iter()
        .map(|rec| {
            let mut someAdd: Option<crate::geoencoding::Address> = None;
            match getAddress(rec.lat, rec.lng) {
                Ok(addr) => {
                    println!("OK: {:?}", addr);
                    someAdd = addr;
                }
                Err(err) => {
                    println!("ERR: {}", err);
                }
                _ => {
                    println!("unkown err");
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
                
                constraints: getConstraint(rec.id, &con.db),
            };
        })
        .collect()
    );
}