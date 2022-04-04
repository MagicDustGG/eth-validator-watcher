use diesel::{
	ExpressionMethods, Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
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
	input: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
	hash: H256,
	block_hash: H256,
	index: u64,
	from: Option<H160>,
	to: Option<H160>,
	input: Vec<u8>,
}

impl From<DbTransaction> for Transaction {
	fn from(db_transaction: DbTransaction) -> Self {
		Transaction {
			hash: db_transaction.hash.into(),
			block_hash: db_transaction.block_hash.into(),
			index: db_transaction.index as u64,
			from: db_transaction.from.map(|f| f.into()),
			to: db_transaction.to.map(|t| t.into()),
			input: db_transaction.input,
		}
	}
}

impl Transaction {
	/// Return an unique transaction from db
	pub fn get(conn: &PgConnection, hash: H256) -> QueryResult<Transaction> {
		let hash: Hash256 = hash.into();
		let transaction: DbTransaction = dsl_transactions.find(hash).first(conn)?;

		Ok(transaction.into())
	}

	pub fn list_from_address(conn: &PgConnection, address: H160) -> QueryResult<Vec<Transaction>> {
		let address: Hash160 = address.into();

		let db_transactions: Vec<DbTransaction> =
			dsl_transactions.filter(transactions::from.eq(address)).load(conn)?;

		let transactions: Vec<Transaction> =
			db_transactions.into_iter().map(|t| t.into()).collect();

		Ok(transactions)
	}

	/// Return the address of the transaction recipient
	pub fn to(&self) -> Option<H160> {
		self.to
	}

	/// Return the transaction input
	pub fn input(&self) -> Vec<u8> {
		self.input.clone()
	}
}
