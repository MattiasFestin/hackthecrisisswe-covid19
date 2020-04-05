#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate redis;
extern crate dotenv;
extern crate geo;
extern crate reqwest;

use rocket_cors;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

mod db_models;
mod view_models;
mod cors_fairing;
mod apihelper;
mod transaction;
mod schema;
mod geoencoding;

use diesel::prelude::*;

fn cors_options() -> CorsOptions {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);

    // You can also deserialize this
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
}

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

    // if envmnt::get_or("DEBUG", "0") != "0" {
        let server = rocket::ignite()
            .attach(crate::cors_fairing::CORS())
            .mount("/", routes)
            .mount("/", rocket_cors::catch_all_options_routes())
            .launch(); // mount the catch all routes
            // .manage(cors_options().to_cors().expect("To not fail"));
    // } else {
    //     let server = rocket::ignite()
    //         .mount("/", routes)
    //         .launch();
    // }

}