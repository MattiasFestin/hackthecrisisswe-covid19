use std::env;
// use std::sync::Arc;
use std::ops::Deref;

// use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;


// use r2d2_redis::{r2d2, RedisConnectionManager};
// use r2d2_redis::redis::Commands;

use rocket::{Outcome, Request, State};
use rocket::request::{self, FromRequest};
use rocket::http::Status;


// //TODO: Move redis connection
// pub fn establish_redis_connection() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
//     dotenv().ok();

//     let url = env::var("REDIS_URL")
// 		.expect("REDIS_URL must be set");

// 	let manager = RedisConnectionManager::new(url).unwrap();
	
// 	r2d2::Pool::builder()
// 		.max_size(10)
// 		.build(manager)
// 		.unwrap()
// }


pub fn establish_db_connection() -> diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

	let manager = diesel::r2d2::ConnectionManager::new(database_url);
	
	diesel::r2d2::Pool::builder()
		.max_size(24)
		.build(manager)
		.unwrap()
}

// This is the struct we will be passing around as a request guard.
// DbConn is a tuple-struct, which only has one field.
// It is accessed with the notation `my_tuple_struct.0` and will serve as a wrapper
// to implent the FromRequest trait on.
pub struct DbConn(diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>);

// Our impl of FromRequest for our DbConn tuple-struct.
// This is what actually enables our connection pool to become
// a request guard.
// Docs: https://api.rocket.rs/rocket/request/trait.FromRequest.html
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = (); // Associated type, we must have an error we can `Debug`

    // This is our required method that does all the dirty work.
    // Because FromRequest is a "validation", we can put whatever logic we want in here
    // as long as it conforms to the function signature.
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {

        // This next part is a little dense, but what it's doing is grabbing the 
        // `guard` property off of the `request` object. From there we have to give
        // it a type, which you'll see in this massive turbofish ::<<<<>>>.
        // ...Might be a world record :P

        // The outside most type is State, which is the managed state we will be registering
        // with our rocket app when we initialize it. Don't worry, we'll get to that soon enough,
        // but you'll have to trust me here.
        let pool = request.guard::<State<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>>()?;

        // Here were are using the `get()` method from the connection pool to grab 
        // the connection. If it's Ok, return the DbConn tuple-struct we made
        // wrapped in an `Outcome` to conform to the function signature.
        // If the `get()` returns an Error, we're returning a tuple with the
        // signature (SomeFailureType, ())
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
