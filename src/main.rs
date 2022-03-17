#[macro_use]
extern crate diesel;

mod beacon_client;
mod error;
mod postgres;

use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use error::*;

use diesel::PgConnection;
use dotenv::dotenv;
use eth2::BeaconNodeHttpClient;
use log::debug;

use crate::postgres::{NewSlot, Slot};
use tokio::time;

async fn bump_slots(
	client: &BeaconNodeHttpClient,
	conn: Arc<Mutex<PgConnection>>,
	from_slot: Option<u64>,
) -> Result<u64, Error> {
	let from_slot = from_slot.unwrap_or_else(|| {
		Slot::get_highest(&conn.lock().unwrap()).map_or(0, |slot| slot.id() + 1)
	});
	let chain_height = beacon_client::get_head_height(client).await?;

	debug!("Starting db bump, from {} to {}", from_slot, chain_height);
	for slot_id in from_slot..chain_height + 1 {
		debug!("Slot {}", slot_id);
		if Slot::get(&conn.lock().unwrap(), slot_id).is_none() {
			let opt_validators = beacon_client::get_validators(client, slot_id).await?;
			NewSlot::new(slot_id, opt_validators.map(|v| v.len())).upsert(&conn.lock().unwrap())?;
			debug!("Slot {} saved", slot_id);
		}
	}

	Ok(chain_height)
}

async fn keep_up_with_chain(
	client: &BeaconNodeHttpClient,
	conn: Arc<Mutex<PgConnection>>,
) -> Result<(), Error> {
	let mut interval_20_sec = time::interval(Duration::from_secs(20));

	loop {
		interval_20_sec.tick().await;
		bump_slots(client, conn.clone(), None).await?;
	}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();

	let conn = Arc::new(Mutex::new(postgres::establish_connection()));
	let client = beacon_client::get_client()?;

	bump_slots(&client, conn.clone(), Some(0)).await?;
	tokio::spawn(async move { keep_up_with_chain(&client, conn.clone()).await });

	Ok(())
}
