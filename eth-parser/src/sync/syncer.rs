use std::{fmt::Display, time::Duration};

use async_trait::async_trait;
use log::{info, warn};
use tokio::time;

use crate::Error;

#[derive(Debug)]
pub enum SyncError {
	/// Block not found at height
	NothingAtHeight(u64),
	/// The indexed block was pending
	PendingBlock(u64),
	/// The client did not return any validators
	NoValidators,
}

#[async_trait]
pub trait DbSyncer: Display {
	type NodeClient;

	/// Keep database synced with the node
	///
	/// Never return. Contains an infinte loop, pooling for new blocks every new tick.
	///
	/// # Arguments
	/// * `from`: height from where to start pulling blocks
	/// * `interval_duration`: interval at which the database will be synced
	///
	/// # Execution
	/// All blocks between `from` and the current node head, included, will be processed.
	/// If `from` is `None`, the database will be queried for it's highest block, and sync will
	/// start from the next one. If the table is empty, sync will start at height 0.
	///
	/// After head is reach for the first time, subsequent attempt to sync will occur every
	/// `interval_duration` and start from the previous registered max height.
	///
	/// Existing entries are not updated
	async fn keep_in_sync(&self, from: Option<u64>, interval_duration: Duration) {
		let mut sync_interval = time::interval(interval_duration);
		let mut from = from;

		let bump = |from: Option<u64>| async move {
			let from_height =
				from.unwrap_or_else(|| self.get_db_height().map_or(0, |slot| slot + 1));
			let chain_height = self.get_node_height().await?;
			if from_height == chain_height {
				return Ok(chain_height) as Result<u64, Error>
			}

			info!("{self}: Bumping database from heigth {from_height} to {chain_height}",);

			for height in from_height..=chain_height {
				match self.create_new_entry(height).await {
					Ok(_) => info!("{self}: Saved entry at height {height}"),
					Err(err) => warn!("{self}: Failed to create enty at height {height}: {err}"),
				}
			}

			Ok(chain_height)
		};

		loop {
			sync_interval.tick().await;
			match bump(from.take()).await {
				Ok(head) => info!("{self}: Database synced up to node. Head is {head}"),
				Err(err) => warn!("{self}: Failed to bump up to head: {err}"),
			}
		}
	}

	/// Return a instance of the node client
	fn node_client(&self) -> Self::NodeClient;

	/// Return the node head height
	async fn get_node_height(&self) -> Result<u64, Error>;
	/// Return the database head height
	fn get_db_height(&self) -> Result<u64, Error>;

	/// Register a new entry in database
	///
	/// # Arguments
	/// * `height`: height of the block to create
	///
	/// Called internaly by `keep_in_sync`.
	/// Should fetch data from the node and store them in database.
	async fn create_new_entry(&self, height: u64) -> Result<(), Error>;
}
