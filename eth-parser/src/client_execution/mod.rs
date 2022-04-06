use std::env;

use web3::{
	transports::Http,
	types::{Block, BlockId, BlockNumber, Transaction, TransactionReceipt, H256},
	Web3,
};

use crate::Error;

/// Create a new Web3 client
///
/// # Environment requirement
/// `EXECUTION_LAYER_URL`: "http://<node_url>:<port>"
pub fn new_client() -> Result<Web3<Http>, Error> {
	let raw_url = env::var("EXECUTION_LAYER_URL")?;
	let url = web3::transports::Http::new(&raw_url)?;

	Ok(web3::Web3::new(url))
}

/// Get the block at `height`
///
/// https://eth.wiki/json-rpc/API#eth_getblockbynumber
pub async fn get_block(
	client: Web3<Http>,
	height: u64,
) -> Result<Option<Block<Transaction>>, Error> {
	let block_id = BlockId::Number(BlockNumber::Number(height.into()));
	let opt_r = client.eth().block_with_txs(block_id).await?;

	Ok(opt_r)
}

/// Get the receipt of transaction `hash`
///
/// https://eth.wiki/json-rpc/API#eth_gettransactionreceipt
pub async fn get_transaction_receipt(
	client: Web3<Http>,
	hash: H256,
) -> Result<Option<TransactionReceipt>, Error> {
	let receip = client.eth().transaction_receipt(hash).await?;

	Ok(receip)
}
