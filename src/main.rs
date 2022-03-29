#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use schema::validators::dsl::*;

use eth2::{
	types::{StateId, ValidatorData},
	BeaconNodeHttpClient as Client, Timeouts,
};
use sensitive_url::SensitiveUrl;

use dotenv::dotenv;
use itertools::Itertools;
use std::{
	env,
	time::{Duration, SystemTime},
};

pub fn establish_connection() -> MysqlConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
	MysqlConnection::establish(&database_url)
		.expect(&format!("Error connecting to {}", database_url))
}

pub fn create_validators<I>(conn: &MysqlConnection, validator_data: I) -> usize
where
	I: Iterator<Item = ValidatorData>,
{
	use schema::validators;

	let new_validators = validator_data.map(|data| data.into()).collect();

	diesel::insert_into(validators::table)
		.values::<Vec<NewValidator>>(new_validators)
		.execute(conn)
		.expect("Error saving new validators")
}

#[tokio::main]
async fn main() -> Result<(), eth2::Error> {
	dotenv().ok();

	let raw_url = env::var("KILN_NODE_URL").expect("KILN_NODE_URL must be set");
	let url = SensitiveUrl::parse(&raw_url).expect("KIL_NODE_URL is not a valid url");

	let client = Client::new(url, Timeouts::set_all(Duration::from_secs(1)));
	let opt_response = client.get_beacon_states_validators(StateId::Head, None, None).await?;

	let connection = establish_connection();

	println!("insertion: {:?}", SystemTime::now());
	match opt_response {
		Some(r) => {
			diesel::delete(validators)
				.execute(&connection)
				.expect("Error deleting validators table content");
			let res_validators = r.data;
			for chunk in &res_validators.into_iter().chunks(1000) {
				create_validators(&connection, chunk);
			}
		},
		None => {},
	}
	println!("done: {:?}", SystemTime::now());

	let results = validators
		.filter(slashed.eq(true))
		.limit(5)
		.load::<Validator>(&connection)
		.expect("Error loading validators");

	println!("{:?}", results);

	Ok(())
}
