use std::fmt::Display;

use async_trait::async_trait;
use log::{info, warn};

use crate::Error;

/// Sugar around storing chain block in database
///
/// Allow for full control over how to pull entry and what to store.
/// Only take care of the looping part.
#[async_trait]
pub trait DbSyncer: Display {
	type NodeClient;

	/// Bump database
	///
	/// Call `create_new_entry` for every height between `from` and `to` included
	///
	/// If from is None, the height following the highest stored height in db will be used.
	/// If db empty from will be 0.
	async fn bump(&self, from: Option<u64>, to: u64) -> Result<u64, Error> {
		let from = from.unwrap_or_else(|| self.get_db_height().map_or(0, |slot| slot + 1));

		info!("{self}: Bumping database from heigth {from} to {to}",);

		for height in from..=to {
			match self.create_new_entry(height).await {
				Ok(()) => info!("{self}: Saved entry at height {height}"),
				Err(err) => warn!("{self}: Failed to create enty at height {height}: {err}"),
			}
		}

		Ok(to)
	}

	/// Return a instance of the node client
	fn node_client(&self) -> Self::NodeClient;

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
