use std::fmt::Display;

use async_trait::async_trait;
use kiln_postgres::{ExecBlock, NewExecBlock, NewTransaction, NewTransactions, PgConnectionPool};
use web3::{transports::Http, types::Transaction, Web3};

use super::syncer::{DbSyncer, SyncError};

use crate::{client_execution, Error};

pub(crate) struct ExecutionSyncer(PgConnectionPool, Web3<Http>);

impl ExecutionSyncer {
	pub fn new(conn: PgConnectionPool, client: Web3<Http>) -> ExecutionSyncer {
		ExecutionSyncer(conn, client)
	}
}

impl Display for ExecutionSyncer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "execution syncer")
	}
}

#[async_trait]
impl DbSyncer for ExecutionSyncer {
	type NodeClient = Web3<Http>;

	fn node_client(&self) -> Self::NodeClient {
		self.1.clone()
	}

	fn get_db_height(&self) -> Result<u64, Error> {
		let block = ExecBlock::get_highest(&self.0.get().unwrap())?;

		Ok(block.number())
	}

	async fn get_node_height(&self) -> Result<u64, Error> {
		client_execution::get_head_height(self.node_client()).await
	}

	async fn create_new_entry(&self, height: u64) -> Result<(), Error> {
		// Get block from client
		let block = client_execution::get_block(self.node_client(), height)
			.await?
			.ok_or(SyncError::NothingAtHeight(height))?;

		// Handle and insert block
		let new_block = NewExecBlock::new(
			block.hash.ok_or(SyncError::PendingBlock(height))?,
			block.number.ok_or(SyncError::PendingBlock(height))?.as_u64(),
			block.parent_hash,
			block.state_root,
			block.transactions_root,
			block.receipts_root,
		);
		new_block.insert(&self.0.get().unwrap())?;

		// Handle and insert transactions
		let new_transactions: NewTransactions = block
			.transactions
			.into_iter()
			.map(|t: Transaction| {
				NewTransaction::new(
					t.hash,
					t.block_hash.unwrap(),
					t.transaction_index.unwrap().as_u64(),
					t.from,
					t.to,
				)
			})
			.collect();
		new_transactions.batch_insert(&self.0.get().unwrap())?;

		Ok(())
	}
}
