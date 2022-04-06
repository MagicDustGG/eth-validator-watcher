use std::fmt::Display;

use async_trait::async_trait;
use ethereum_abi::Abi;
use futures::future::try_join_all;
use kiln_postgres::{
	ExecBlock, NewExecBlock, NewTransaction, NewTransactions, NewValidator, PgConnectionPool,
};
use log::{debug, error};
use web3::{
	transports::Http,
	types::{Transaction, H160, H256},
	Web3,
};

use super::{syncer::DbSyncer, SyncError};

use crate::{client_execution, Error};

// The deposit contract address for the kiln network
//
// # value
// 0x4242424242424242424242424242424242424242
//
// # explorer
// https://explorer.kiln.themerge.dev/address/0x4242424242424242424242424242424242424242/transactions
const DEPOSIT_CONTRACT_ADDRESS: [u8; 20] = [
	0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
	0x42, 0x42, 0x42, 0x42,
];

lazy_static! {
	static ref DEPOSIT_CONTRACT_ABI: Abi = serde_json::from_str(r#"[{"inputs":[],"stateMutability":"nonpayable","type":"constructor"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"bytes","name":"pubkey","type":"bytes"},{"indexed":false,"internalType":"bytes","name":"withdrawal_credentials","type":"bytes"},{"indexed":false,"internalType":"bytes","name":"amount","type":"bytes"},{"indexed":false,"internalType":"bytes","name":"signature","type":"bytes"},{"indexed":false,"internalType":"bytes","name":"index","type":"bytes"}],"name":"DepositEvent","type":"event"},{"inputs":[{"internalType":"bytes","name":"pubkey","type":"bytes"},{"internalType":"bytes","name":"withdrawal_credentials","type":"bytes"},{"internalType":"bytes","name":"signature","type":"bytes"},{"internalType":"bytes32","name":"deposit_data_root","type":"bytes32"}],"name":"deposit","outputs":[],"stateMutability":"payable","type":"function"},{"inputs":[],"name":"get_deposit_count","outputs":[{"internalType":"bytes","name":"","type":"bytes"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"get_deposit_root","outputs":[{"internalType":"bytes32","name":"","type":"bytes32"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"bytes4","name":"interfaceId","type":"bytes4"}],"name":"supportsInterface","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"pure","type":"function"}]"#).unwrap();
}

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

		// async calls to execute after all new transactions are stored in db
		let mut futures = vec![];

		// Handle and insert transactions
		let mut new_transactions = Vec::with_capacity(block.transactions.len());
		block.transactions.into_iter().for_each(|t: Transaction| {
			match t.to {
				Some(to) if to == H160::from(DEPOSIT_CONTRACT_ADDRESS) => futures.push(
					link_validator_to_depositor(self.node_client(), self.0.clone(), t.clone()),
				),
				_ => {},
			};

			new_transactions.push(NewTransaction::new(
				t.hash,
				t.block_hash.unwrap(),
				t.transaction_index.unwrap().as_u64(),
				t.from,
				t.to,
				t.input.0,
				t.value,
			));
		});

		NewTransactions::new(new_transactions).batch_insert(&self.0.get().unwrap())?;

		try_join_all(futures).await?;

		Ok(())
	}
}

// Create a link in database between a validator and the successful calls to the deposit contract
// that registered it
async fn link_validator_to_depositor(
	client: Web3<Http>,
	conn_pool: PgConnectionPool,
	transaction: Transaction,
) -> Result<(), Error> {
	let (function, decoded_params) =
		match DEPOSIT_CONTRACT_ABI.decode_input_from_slice(&transaction.input.0) {
			Ok(d) => d,
			Err(_) => return Ok(()),
		};

	if function.name != "deposit" {
		return Ok(())
	}

	let status = is_transaction_successful(client, transaction.hash).await?;
	NewTransaction::set_status(&conn_pool.get().unwrap(), transaction.hash, status)?;

	let bytes = match &decoded_params.get(0).unwrap().value {
		ethereum_abi::Value::Bytes(b) => b,
		_ => return Ok(()),
	};
	let mut pubkey = "0x".to_string();
	pubkey.push_str(&hex::encode(bytes));

	let transaction_hash = transaction.hash;
	let rows =
		NewValidator::set_deposit_transaction(&conn_pool.get().unwrap(), pubkey, transaction_hash)?;
	debug!("transactin: {:?}", transaction.from);
	if rows != 1 {
		error!(
			"wrong amount ({rows}) of validators are linked to the transaction {:?}",
			transaction.hash,
		);
	}

	Ok(())
}

// Check transaction status on reciept
//
// # Safety
// Will panic if called on a transaction that have not been included (pending)
async fn is_transaction_successful(
	client: Web3<Http>,
	transaction_hash: H256,
) -> Result<bool, Error> {
	let reciept = client_execution::get_transaction_receipt(client, transaction_hash)
		.await?
		// Only safe to unwrap because the transaction have been included
		.unwrap();

	// Safe to unwrap because kiln is post Byzantum
	let status = reciept.status.unwrap();

	Ok(!status.is_zero())
}
