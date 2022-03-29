mod models;

mod slots;

use std::env;

use diesel::{Connection, PgConnection};

pub use models::{NewSlot, Slot};

pub fn establish_connection() -> PgConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
