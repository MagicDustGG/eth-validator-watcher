use diesel::{Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};

use crate::{
	models::{Hash160, Hash256},
	schema::{transactions, transactions::dsl::transactions as dsl_transactions},
};

#[derive(Queryable, Identifiable)]
#[primary_key(hash)]
#[table_name = "transactions"]
struct DbTransaction {
	hash: Hash256,
	block_hash: Hash256,
	index: i64,
	from: Option<Hash160>,
	to: Option<Hash160>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
	hash: H256,
	block_hash: H256,
	index: u64,
	from: Option<H160>,
	to: Option<H160>,
}

impl From<DbTransaction> for Transaction {
	fn from(db_transaction: DbTransaction) -> Self {
		Transaction {
			hash: db_transaction.hash.into(),
			block_hash: db_transaction.block_hash.into(),
			index: db_transaction.index as u64,
			from: db_transaction.from.map(|f| f.into()),
			to: db_transaction.to.map(|t| t.into()),
		}
	}
}

impl Transaction {
	/// Return an unique transaction from db
	pub fn get(conn: &PgConnection, hash: H256) -> QueryResult<Transaction> {
		let hash: Hash256 = hash.into();
		let block: DbTransaction = dsl_transactions.find(hash).first(conn)?;

		Ok(block.into())
	}
}
