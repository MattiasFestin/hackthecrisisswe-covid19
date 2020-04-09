#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate geo;
extern crate reqwest;
extern crate r2d2_redis;

use rocket_cors;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

mod db;
mod redis;
mod db_models;
mod view_models;
mod cors_fairing;
mod transaction;
mod schema;
mod geoencoding;

use diesel::prelude::*;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let server = rocket::ignite();

    let routes = routes![
        index,
        crate::transaction::getTransaction,
        crate::transaction::getTransactionList,
        crate::transaction::insertTransaction
    ];

    if envmnt::get_or("DEBUG", "0") != "0" {
        let server = rocket::ignite()
            .attach(crate::cors_fairing::CORS())
            .mount("/", routes)
            
            .mount("/", rocket_cors::catch_all_options_routes())
            .manage(crate::db::establish_db_connection())
            .manage(crate::redis::establish_redis_connection())

            .launch();
    } else {
        let server = rocket::ignite()
            .mount("/", routes)
            
            .mount("/", rocket_cors::catch_all_options_routes())
            .manage(crate::db::establish_db_connection())
            .manage(crate::redis::establish_redis_connection())

            .launch();
    }

}