use diesel::{PgConnection, QueryResult, RunQueryDsl};
use primitive_types::H256;

use crate::{models::Hash256, schema::transactions};

#[derive(Insertable)]
#[table_name = "transactions"]
pub struct NewTransaction {
	hash: Hash256,
	block_hash: Hash256,
	index: i64,
	from: Option<Hash256>,
	to: Option<Hash256>,
}

impl NewTransaction {
	/// Return a new insertable Transaction
	pub fn new(
		hash: H256,
		block_hash: H256,
		index: u64,
		from: Option<H256>,
		to: Option<H256>,
	) -> NewTransaction {
		NewTransaction {
			hash: hash.into(),
			block_hash: block_hash.into(),
			index: index as i64,
			from: from.map(|f| f.into()),
			to: to.map(|t| t.into()),
		}
	}

	/// Insert a new transaction on db
	///
	/// Fail in case of conflict
	pub fn insert(&self, conn: &PgConnection) -> QueryResult<usize> {
		diesel::insert_into(transactions::table).values(self).execute(conn)
	}
}
