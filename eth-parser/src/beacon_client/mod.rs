use std::{env, time::Duration};

use eth2::{
	types::{ConfigAndPreset, MainnetEthSpec, SignedBeaconBlock, Slot, StateId, ValidatorData},
	BeaconNodeHttpClient, Timeouts,
};
use sensitive_url::SensitiveUrl;

use crate::Error;

/// Create a new Beacon client
///
/// # Environment requirement
/// `CONSENSUS_LAYER_URL`: "http://<node_url>:<port>"
pub fn new_client() -> Result<BeaconNodeHttpClient, Error> {
	let raw_url = env::var("CONSENSUS_LAYER_URL")?;
	let url = SensitiveUrl::parse(&raw_url)?;

	Ok(BeaconNodeHttpClient::new(
		url,
		Timeouts::set_all(Duration::from_secs(1)),
	))
}

/// Return the id of the highest slot synced by the node
///
/// https://ethereum.github.io/beacon-APIs/#/Node/getSyncingStatus response.head_slot
pub async fn get_head_height(client: &BeaconNodeHttpClient) -> Result<u64, Error> {
	let ret = client.get_node_syncing().await?;

	Ok(ret.data.head_slot.as_u64())
}

/// Return the list of validators at a given slot
///
/// https://ethereum.github.io/beacon-APIs/#/Beacon/getStateValidators
pub async fn get_validators_at_slot(
	client: &BeaconNodeHttpClient,
	slot_id: u64,
) -> Result<Option<Vec<ValidatorData>>, Error> {
	let state_id = StateId::Slot(Slot::new(slot_id));
	let opt_r = client.get_beacon_states_validators(state_id, None, None).await?;

	Ok(opt_r.map(|r| r.data))
}

/// Return the chain spec
///
/// https://ethereum.github.io/beacon-APIs/#/Config/getSpec
pub async fn get_config_spec(client: &BeaconNodeHttpClient) -> Result<ConfigAndPreset, Error> {
	let r = client.get_config_spec().await?;

	Ok(r.data)
}

/// Return the block at `slot_height`
///
/// https://ethereum.github.io/beacon-APIs/#/Beacon/getBlockV2
#[allow(dead_code)]
pub async fn get_block(
	client: &BeaconNodeHttpClient,
	slot_height: u64,
) -> Result<Option<SignedBeaconBlock<MainnetEthSpec>>, Error> {
	let block_id = eth2::types::BlockId::Slot(Slot::new(slot_height));
	let opt_r = client.get_beacon_blocks::<MainnetEthSpec>(block_id).await?;

	Ok(opt_r.map(|r| r.data))
}
