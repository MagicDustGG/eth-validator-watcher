use std::env;

use web3::{
	transports::Http,
	types::{Block, BlockId, BlockNumber, H256},
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

#[allow(dead_code)]
/// Get the block at `height`
///
/// https://eth.wiki/json-rpc/API#eth_getblockbynumber
pub async fn get_block(client: Web3<Http>, height: u64) -> Result<Option<Block<H256>>, Error> {
	let block_id = BlockId::Number(BlockNumber::Number(height.into()));
	let r = client.eth().block(block_id).await?;

	Ok(r)
}
