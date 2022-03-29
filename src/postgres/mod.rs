mod models;

mod slots;

use std::env;

use diesel::{Connection, PgConnection};

pub use models::{NewSlot, Slot};

/// Establish a connection to a Postgres database
///
/// # Environment requirements
/// `DATABASE_URL`: "postgres:<username>:<password>:<host_url>:<port>/<db_name>"
pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url)
		.unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
