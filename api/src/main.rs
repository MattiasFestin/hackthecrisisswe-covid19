#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate redis;
extern crate dotenv;
extern crate geo;

mod db_models;
mod cors_fairing;
mod apihelper;

use crate::db_models::Users;

#[get("/")]
fn index() -> &'static str {
    let con = crate::apihelper::connect();
    "Hello, world!"
}

fn main() {
    let mut server = rocket::ignite().mount("/", routes![
        index
    ]);

    if envmnt::get_or("DEBUG", "0") != "0" {
        server
            .attach(crate::cors_fairing::CORS())
            .launch();
    } else {
        server.launch();
    }
}