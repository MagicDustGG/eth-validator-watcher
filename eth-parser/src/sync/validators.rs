use std::time::Duration;

use eth2::BeaconNodeHttpClient;
use kiln_postgres::{NewValidators, PgConnectionPool};
use log::{info, warn};
use tokio::time;

use crate::{client_consensus, error::Error};

use super::SyncError;

/// Keep the validator table in sync with the last state of the chain
///
/// Never return due to infinite loop
pub async fn sync_last_validator_state(
	conn_pool: PgConnectionPool,
	client: BeaconNodeHttpClient,
	interval_duration: Duration,
) {
	let mut sync_interval = time::interval(interval_duration);
	loop {
		sync_interval.tick().await;
		match update_validators(conn_pool.clone(), &client).await {
			Ok(_) => info!("Validators updated"),
			Err(err) => warn!("Failed to update validators: {err}"),
		}
	}
}

async fn update_validators(
	conn_pool: PgConnectionPool,
	client: &BeaconNodeHttpClient,
) -> Result<(), Error> {
	let validators = client_consensus::get_validators_at_head(client)
		.await?
		.ok_or(SyncError::NoValidators)?;

	let new_validators = NewValidators::from_iter(validators.into_iter().map(|v| v.into()));
	new_validators.batch_upsert(&conn_pool.get().unwrap())?;

	Ok(())
}
