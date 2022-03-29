mod args;
mod client_consensus;
mod client_execution;
mod error;
mod sync;

use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use args::Args;
use clap::StructOpt;
use dotenv::dotenv;
use error::*;
use tokio::join;

use crate::sync::{ConsensusSyncer, DbSyncer, ExecutionSyncer};

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();
	let args = Args::parse();

	let conn = Arc::new(Mutex::new(kiln_postgres::establish_connection()));
	let eth2 = client_consensus::new_client()?;
	let web3 = client_execution::new_client()?;

	let spec = client_consensus::get_config_spec(&eth2).await?;
	let config = spec.config;
	// Currently we only  handle the mainet preset
	if config.preset_base != "mainnet" {
		return Err(Error::InvalidChainPreset(config.preset_base))
	}
	// Handling multiple chains would add a lot of complexity in database (each block must reference
	// the chain it's related to). Only handling Kiln is all we need right now
	match config.config_name {
		Some(name) if name != "kiln" => return Err(Error::InvalidChainName),
		None => return Err(Error::MissingChainName),
		_ => {},
	}

	let consensus_syncer = ConsensusSyncer::new(conn.clone(), eth2);
	let execution_syncer = ExecutionSyncer::new(conn.clone(), web3);

	// Never return due to infinite loops
	join!(
		consensus_syncer.keep_in_sync(args.from_slot(), Duration::from_secs(20)),
		execution_syncer.keep_in_sync(args.from_block(), Duration::from_secs(20))
	);

	Ok(())
}
