mod beacon_client;
mod consensus_layer;
mod error;
mod execution_layer;
mod traits;
mod web3_client;

use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use consensus_layer::ConsensusSyncer;
use error::*;

use dotenv::dotenv;

use execution_layer::ExecutionSyncer;
use tokio::join;
use traits::DbSyncer;

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();

	let conn = Arc::new(Mutex::new(kiln_postgres::establish_connection()));
	let eth2 = beacon_client::new_client()?;
	let web3 = web3_client::new_client()?;

	let spec = beacon_client::get_config_spec(&eth2).await?;
	let config = spec.config;
	if config.preset_base != "mainnet" {
		return Err(Error::InvalidChainPreset(config.preset_base))
	}
	match config.config_name {
		Some(name) if name != "kiln" => return Err(Error::InvalidChainName),
		None => return Err(Error::MissingChainName),
		_ => {},
	}

	let consensus_syncer = ConsensusSyncer::new(conn.clone(), eth2);
	let execution_syncer = ExecutionSyncer::new(conn.clone(), web3);

	let consensus_handle = consensus_syncer.sync_to_head(None, Duration::from_secs(20));
	let execution_handle = execution_syncer.sync_to_head(None, Duration::from_secs(20));

	join!(consensus_handle, execution_handle);

	Ok(())
}
