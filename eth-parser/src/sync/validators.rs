use eth2::BeaconNodeHttpClient;
use kiln_postgres::{NewValidators, PgConnectionPool};
use log::info;

use crate::{client_consensus, error::Error};

use super::SyncError;

/// Update db validators
pub async fn update_validators(
	conn_pool: PgConnectionPool,
	client: &BeaconNodeHttpClient,
	slot: u64,
) -> Result<(), Error> {
	info!("syncing db with validators at slot {slot}");

	let validators = client_consensus::get_validators_at_slot(client, slot)
		.await?
		.ok_or(SyncError::NoValidators)?;

	let new_validators = NewValidators::from_iter(validators.into_iter().map(|v| v.into()));
	new_validators.batch_upsert(&conn_pool.get().unwrap())?;

	Ok(())
}
