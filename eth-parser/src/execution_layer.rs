use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use diesel::PgConnection;
use kiln_postgres::{ExecBlock, NewExecBlock};
use web3::{transports::Http, Web3};

use crate::{
	traits::{DbSyncer, SyncError},
	web3_client, Error,
};

pub(crate) struct ExecutionSyncer(Arc<Mutex<PgConnection>>, Web3<Http>);

impl ExecutionSyncer {
	pub fn new(conn: Arc<Mutex<PgConnection>>, client: Web3<Http>) -> ExecutionSyncer {
		ExecutionSyncer(conn, client)
	}
}

#[async_trait]
impl DbSyncer for ExecutionSyncer {
	type DbConnection = PgConnection;
	type NodeClient = Web3<Http>;

	fn name(&self) -> String {
		"execution Layer".to_owned()
	}

	fn db_conn(&self) -> Arc<Mutex<Self::DbConnection>> {
		self.0.clone()
	}

	fn node_client(&self) -> Self::NodeClient {
		self.1.clone()
	}

	fn get_db_height(&self) -> Result<u64, Error> {
		let block = ExecBlock::get_highest(&self.db_conn().lock().unwrap())?;

		Ok(block.number())
	}

	async fn get_node_height(&self) -> Result<u64, Error> {
		web3_client::get_head_height(self.node_client()).await
	}

	async fn create_new_entry(&self, height: u64) -> Result<(), Error> {
		let block = web3_client::get_block(self.node_client(), height)
			.await?
			.ok_or(SyncError::NothingAtHeight(height))?;

		let new_block = NewExecBlock::new(
			block.hash.ok_or(SyncError::PendingBlock(height))?,
			block.number.ok_or(SyncError::PendingBlock(height))?.as_u64(),
			block.parent_hash,
			block.state_root,
			block.transactions_root,
			block.receipts_root,
		);

		new_block.insert(&self.db_conn().lock().unwrap())?;

		Ok(())
	}
}
