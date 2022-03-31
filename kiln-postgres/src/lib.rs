#[macro_use]
extern crate diesel;

mod models;
mod schema;

use std::env;

use diesel::{
	r2d2::{self, ConnectionManager, Pool},
	PgConnection,
};

pub use models::*;

pub type PgConnectionPool = Pool<ConnectionManager<PgConnection>>;

/// Return a pool of connections to a Postgres instance
///
/// # Environment requirements
/// DATABASE_URL="postgres:<username>:<password>:<host_url>:<port>/<db_name>"
pub fn connexion_pool() -> PgConnectionPool {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	let manager = r2d2::ConnectionManager::<PgConnection>::new(&database_url);

	r2d2::Pool::new(manager)
		.unwrap_or_else(|_| panic!("Failed to create a pool for database at {}", database_url))
}
