use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub struct ApiIO {
	pub redis: redis::RedisResult<redis::Client>,
	pub db: PgConnection
}

pub fn connect() -> ApiIO {
	return ApiIO {
		redis: establish_redis_connection(),
		db: establish_db_connection()
	};
}

fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
		.expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn establish_redis_connection() -> redis::RedisResult<redis::Client> {
    dotenv().ok();

    let url = env::var("REDIS_URL")
		.expect("REDIS_URL must be set");

	return redis::Client::open(url);
}