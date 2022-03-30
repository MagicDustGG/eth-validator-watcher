use diesel::{
	ExpressionMethods, Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

use crate::{
	models::Hash256,
	schema::{
		execution_blocks,
		execution_blocks::{dsl::execution_blocks as dsl_blocks, number},
	},
};

#[derive(Queryable, Identifiable)]
#[primary_key(hash)]
#[table_name = "execution_blocks"]
struct DbExecBlock {
	hash: Hash256,
	number: i64,
	parent_hash: Hash256,
	state_root: Hash256,
	transactions_root: Hash256,
	receipts_root: Hash256,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExecBlock {
	hash: H256,
	number: u64,
	parent_hash: H256,
	state_root: H256,
	transactions_root: H256,
	receipts_root: H256,
}

impl From<DbExecBlock> for ExecBlock {
	fn from(db_block: DbExecBlock) -> Self {
		ExecBlock {
			hash: db_block.hash.into(),
			number: db_block.number as u64,
			parent_hash: db_block.parent_hash.into(),
			state_root: db_block.state_root.into(),
			transactions_root: db_block.transactions_root.into(),
			receipts_root: db_block.receipts_root.into(),
		}
	}
}

impl ExecBlock {
	/// Return the highest block from db
	pub fn get_highest(conn: &PgConnection) -> QueryResult<ExecBlock> {
		let block = dsl_blocks.order(execution_blocks::number.desc()).first::<DbExecBlock>(conn)?;

		Ok(block.into())
	}

	/// Return an unique block from db
	pub fn get(conn: &PgConnection, height: u64) -> QueryResult<ExecBlock> {
		let block = dsl_blocks.filter(number.eq(height as i64)).first::<DbExecBlock>(conn)?;

		Ok(block.into())
	}

	pub fn number(&self) -> u64 {
		self.number
	}
}
