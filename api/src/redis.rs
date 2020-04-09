use std::env;
use std::sync::Arc;
use std::ops::{Deref, DerefMut};

use diesel::prelude::*;
use dotenv::dotenv;


use r2d2_redis::{r2d2, RedisConnectionManager};
use r2d2_redis::redis::Commands;

use rocket::{Outcome, Request, State};
use rocket::request::{self, FromRequest};
use rocket::http::Status;


//TODO: Move redis connection
pub fn establish_redis_connection() -> r2d2::Pool<r2d2_redis::RedisConnectionManager> {
    dotenv().ok();

    let url = env::var("REDIS_URL")
		.expect("REDIS_URL must be set");

	let manager = RedisConnectionManager::new(url).unwrap();
	
	r2d2::Pool::builder()
		.max_size(24)
		.build(manager)
		.unwrap()
}

pub struct RedisConn(r2d2_redis::r2d2::PooledConnection<r2d2_redis::RedisConnectionManager>);


impl<'a, 'r> FromRequest<'a, 'r> for RedisConn {
    type Error = (); // Associated type, we must have an error we can `Debug`

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RedisConn, ()> {

        let pool = request.guard::<State<r2d2::Pool<r2d2_redis::RedisConnectionManager>>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(RedisConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for RedisConn {
    type Target = r2d2_redis::redis::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RedisConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}