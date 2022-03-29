#[macro_use]
extern crate diesel;

mod beacon_client;
mod error;
mod postgres;

use error::*;

use diesel::PgConnection;
use dotenv::dotenv;
use eth2::BeaconNodeHttpClient;
use log::debug;

use crate::postgres::{NewSlot, Slot};

async fn bump_slots(client: &BeaconNodeHttpClient, conn: &PgConnection) -> Result<(), Error> {
	let latest_slot_in_db = Slot::get_highest(conn).map_or(0, |slot| slot.id());
	let latest_slot_sync = client.get_node_syncing().await?.data.head_slot.as_u64();

	debug!(
		"Starting db bump, from {} to {}",
		latest_slot_in_db, latest_slot_sync
	);
	for slot_id in latest_slot_in_db + 1..latest_slot_sync + 1 {
		if Slot::get(conn, slot_id).is_none() {
			let opt_validators = beacon_client::get_validators(client, slot_id).await?;
			NewSlot::new(slot_id, opt_validators.map(|v| v.len())).upsert(conn)?;
			debug!("Slot {} saved", slot_id);
		}
	}

	Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();

	let connection = postgres::establish_connection();
	let client = beacon_client::get_client()?;

	bump_slots(&client, &connection).await?;

	Ok(())
}
