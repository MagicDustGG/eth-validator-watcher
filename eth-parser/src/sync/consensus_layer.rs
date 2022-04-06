use std::fmt::Display;

use async_trait::async_trait;
use eth2::BeaconNodeHttpClient;
use kiln_postgres::{NewSlot, PgConnectionPool, Slot};
use log::info;

use super::syncer::DbSyncer;

use crate::{client_consensus, Error};

pub(crate) struct ConsensusSyncer(PgConnectionPool, BeaconNodeHttpClient);

impl ConsensusSyncer {
	pub fn new(
		pg_connection: PgConnectionPool,
		client_consensus: BeaconNodeHttpClient,
	) -> ConsensusSyncer {
		ConsensusSyncer(pg_connection, client_consensus)
	}
}

impl Display for ConsensusSyncer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "consensus syncer")
	}
}

#[async_trait]
impl DbSyncer for ConsensusSyncer {
	type NodeClient = BeaconNodeHttpClient;

	fn node_client(&self) -> Self::NodeClient {
		self.1.clone()
	}

	fn get_db_height(&self) -> Result<u64, Error> {
		let highest_slot = Slot::get_highest(&self.0.get().unwrap())?;

		Ok(highest_slot.height())
	}

	async fn create_new_entry(&self, height: u64) -> Result<(), Error> {
		// Fetch block
		let opt_block = client_consensus::get_block(&self.node_client(), height).await?;
		let block = match opt_block {
			Some(b) => b,
			None => {
				info!("Slot {height} was missed");
				return Ok(())
			},
		};

		// Retrieve block hash and block number from the block
		let block_hash = block
			.message()
			.body()
			.execution_payload()
			.ok()
			.map(|p| p.block_hash.into_root());
		let block_number = block.message().body().execution_payload().ok().map(|p| p.block_number);

		// Create a new slot
		let new_slot = NewSlot::new(height, block_hash, block_number);

		// Write the new slot in database
		new_slot.insert_do_nothing(&self.0.get().unwrap())?;

		Ok(())
	}
}
