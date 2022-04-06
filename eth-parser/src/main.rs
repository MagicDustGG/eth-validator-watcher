#[macro_use]
extern crate lazy_static;

mod args;
mod client_consensus;
mod client_execution;
mod error;
mod sync;

use args::Args;
use clap::StructOpt;
use dotenv::dotenv;
use error::*;
use eth2::BeaconNodeHttpClient;
use log::info;
use sync::validators::update_validators;
use tokio::join;

use crate::sync::{ConsensusSyncer, DbSyncer, ExecutionSyncer};

const FIRST_SLOT_WITH_EXEC_BLOCK: u64 = 29151;

#[tokio::main]
async fn main() -> Result<(), Error> {
	dotenv().ok();
	env_logger::init();
	let args = Args::parse();

	if args.freeze_at() < FIRST_SLOT_WITH_EXEC_BLOCK {
		return Err(Error::PreMergeFreezeSlot)
	}

	let conn_pool = kiln_postgres::connexion_pool();
	let eth2 = client_consensus::new_client()?;
	let web3 = client_execution::new_client()?;

	let spec = client_consensus::get_config_spec(&eth2).await?;
	let config = spec.config;
	// Currently we only handle the mainet preset
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

	let mut consensus_height: u64;

	// Sync db with chain height
	// Will loop until heigh rejoin `freeze_at`
	loop {
		consensus_height = client_consensus::get_head_height(&eth2).await?;
		let max_consensus_height = std::cmp::min(consensus_height, args.freeze_at());

		update_validators(conn_pool.clone(), &eth2, max_consensus_height).await?;

		let max_exec_height = find_last_exec_block(&eth2, max_consensus_height).await?;

		let consensus_syncer = ConsensusSyncer::new(conn_pool.clone(), eth2.clone());
		let execution_syncer = ExecutionSyncer::new(conn_pool.clone(), web3.clone());

		let (res_consensus, res_execution) = join!(
			consensus_syncer.bump(args.first_slot().take(), max_consensus_height),
			execution_syncer.bump(args.first_block().take(), max_exec_height),
		);
		res_execution?;
		if res_consensus? == args.freeze_at() {
			break
		}
	}

	Ok(())
}

// Query consensus layer for slot between `height` and 0 until it find one with a non None
// execution_payload
async fn find_last_exec_block(eth2: &BeaconNodeHttpClient, height: u64) -> Result<u64, Error> {
	for h in (0..height + 1).rev() {
		info!("looking for execution payload in slot {h}");
		let slot = match client_consensus::get_block(eth2, h).await? {
			Some(s) => s,
			None => continue,
		};
		let opt_block_number =
			slot.message().body().execution_payload().ok().map(|p| p.block_number);
		if let Some(n) = opt_block_number {
			return Ok(n)
		}
	}

	Ok(0)
}
