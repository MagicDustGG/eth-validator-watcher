use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diesel::PgConnection;
use eth2::BeaconNodeHttpClient;
use kiln_postgres::{NewSlot, Slot};

use super::syncer::{DbSyncer, SyncError};

use crate::{client_consensus, Error};

pub(crate) struct ConsensusSyncer(Arc<Mutex<PgConnection>>, BeaconNodeHttpClient);

impl ConsensusSyncer {
	pub fn new(
		pg_connection: Arc<Mutex<PgConnection>>,
		client_consensus: BeaconNodeHttpClient,
	) -> ConsensusSyncer {
		ConsensusSyncer(pg_connection, client_consensus)
	}
}

#[async_trait]
impl DbSyncer for ConsensusSyncer {
	type DbConnection = PgConnection;
	type NodeClient = BeaconNodeHttpClient;

	fn name(&self) -> String {
		"consensus layer".to_owned()
	}

	fn db_conn(&self) -> Arc<Mutex<Self::DbConnection>> {
		self.0.clone()
	}

	fn node_client(&self) -> Self::NodeClient {
		self.1.clone()
	}

	async fn get_node_height(&self) -> Result<u64, Error> {
		client_consensus::get_head_height(&self.node_client()).await
	}

	fn get_db_height(&self) -> Result<u64, Error> {
		let highest_slot = Slot::get_highest(&self.db_conn().lock().unwrap())?;

		Ok(highest_slot.height())
	}

	async fn create_new_entry(&self, height: u64) -> Result<(), Error> {
		// Fetch validators
		let validators = client_consensus::get_validators_at_slot(&self.node_client(), height)
			.await?
			.ok_or(SyncError::NothingAtHeight(height))?;
		// Fetch block
		let block = client_consensus::get_block(&self.node_client(), height)
			.await?
			.ok_or(SyncError::NothingAtHeight(height))?;

		// Retrieve block hash and block number from the block
		let block_hash = block
			.message()
			.body()
			.execution_payload()
			.ok()
			.map(|p| p.block_hash.into_root());
		let block_number = block.message().body().execution_payload().ok().map(|p| p.block_number);

		// Create a new slot
		let new_slot = NewSlot::new(height, validators.len(), block_hash, block_number);

		// Write the new slot in database
		new_slot.insert_do_nothing(&self.db_conn().lock().unwrap())?;

		Ok(())
	}
}
