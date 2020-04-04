use std::time::SystemTime;
use serde::Serialize;
use uuid::Uuid;
// use rocket_contrib::uuid::Uuid;
use crate::diesel::*;


#[get("/transaction?<id>")]
pub fn getTransaction(id: rocket_contrib::uuid::Uuid) -> rocket_contrib::json::Json<Option<crate::db_models::Transaction>> {
    
    let con = crate::apihelper::connect();    
    
    let dbId = Uuid::from_bytes(id.as_bytes()).unwrap();
    
    let mut results: Vec<crate::db_models::Transaction> = crate::schema::transactions::table
        .find(dbId)
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");

    return rocket_contrib::json::Json(results.pop());
}

#[get("/transactions")]
pub fn getTransactionList() -> rocket_contrib::json::Json<Vec<crate::db_models::Transaction>> {
    use crate::schema::transactions::dsl::*;

    let con = crate::apihelper::connect();    
    
    let results: Vec<crate::db_models::Transaction> = transactions.limit(1)
        .load::<crate::db_models::Transaction>(&con.db)
        .expect("Error loading posts");


    return rocket_contrib::json::Json(results);
}