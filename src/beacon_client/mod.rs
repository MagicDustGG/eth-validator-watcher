use std::{env, time::Duration};

use eth2::{
	types::{Slot, StateId, ValidatorData},
	BeaconNodeHttpClient, Timeouts,
};
use sensitive_url::SensitiveUrl;

use crate::Error;

pub fn get_client() -> Result<BeaconNodeHttpClient, Error> {
	let raw_url = env::var("KILN_NODE_URL")?;
	let url = SensitiveUrl::parse(&raw_url)?;

	Ok(BeaconNodeHttpClient::new(
		url,
		Timeouts::set_all(Duration::from_secs(1)),
	))
}

pub async fn get_head_height(client: &BeaconNodeHttpClient) -> Result<u64, Error> {
	let ret = client.get_node_syncing().await?;

	Ok(ret.data.head_slot.as_u64())
}

pub async fn get_validators(
	client: &BeaconNodeHttpClient,
	slot_id: u64,
) -> Result<Option<Vec<ValidatorData>>, Error> {
	let opt_r = client
		.get_beacon_states_validators(StateId::Slot(Slot::new(slot_id)), None, None)
		.await?;

	Ok(opt_r.map(|r| r.data))
}
