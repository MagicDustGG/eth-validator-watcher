mod beacon_client;
mod error;

use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use error::*;

use diesel::PgConnection;
use dotenv::dotenv;
use eth2::BeaconNodeHttpClient;
use log::{info, warn};

use kiln_postgres::{NewSlot, NewSpec, Slot};
use tokio::time;

/// Query new and store them on db
///
/// All slots between `from_slot` included and the head slot included will be processed.
/// If `from_slot` is `None`, the highest slot in db + 1 will be used.
/// If the table is empty querying will start at slot 0.
///
/// If a slot is already in db it will be skipped.
async fn bump_slots(
	client: &BeaconNodeHttpClient,
	conn: Arc<Mutex<PgConnection>>,
	from_slot: Option<u64>,
) -> Result<u64, Error> {
	let from_slot = from_slot.unwrap_or_else(|| {
		Slot::get_highest(&conn.lock().unwrap(), "kiln".to_string())
			.map_or(0, |slot| slot.height() + 1)
	});
	let chain_height = beacon_client::get_head_height(client).await?;
	if from_slot == chain_height {
		return Ok(chain_height)
	}

	info!(
		"Bumping database from slots {} to {}",
		from_slot, chain_height
	);

	for slot_height in from_slot..chain_height + 1 {
		if Slot::get(&conn.lock().unwrap(), "kiln".to_string(), slot_height).is_err() {
			let opt_validators = beacon_client::get_validators_at_slot(client, slot_height).await?;
			NewSlot::new(
				"kiln".to_string(),
				slot_height,
				opt_validators.map(|v| v.len()),
			)
			.upsert(&conn.lock().unwrap())?;
			info!("Saved slot {}", slot_height);
		}
	}

	Ok(chain_height)
}

/// Keep the db up to date with the node
///
/// Bump the db up to the node height evey `inteval_duration`
async fn sync_to_head(
	client: &BeaconNodeHttpClient,
	conn: Arc<Mutex<PgConnection>>,
	interval_duration: Duration,
) -> ! {
	let mut sync_interval = time::interval(interval_duration);

	loop {
		sync_interval.tick().await;
		match bump_slots(client, conn.clone(), None).await {
			Ok(head) => info!("Database synced up to node. Head slot: {}", head),
			Err(err) => warn!("Failed to bump slots up to head: {}", err),
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();

	let conn = Arc::new(Mutex::new(kiln_postgres::establish_connection()));
	let client = beacon_client::new_kiln_client()?;

	let spec = beacon_client::get_config_spec(&client).await?;
	let preset = spec.config.preset_base;
	if preset != "mainnet" {
		return Err(Error::InvalidChainPreset(preset))
	}

	NewSpec::new(
		&spec.config.config_name.ok_or(Error::MissingChainName)?,
		&preset,
	)
	.upsert(&conn.lock().unwrap())?;

	sync_to_head(&client, conn.clone(), Duration::from_secs(20)).await;

	Ok(())
}
