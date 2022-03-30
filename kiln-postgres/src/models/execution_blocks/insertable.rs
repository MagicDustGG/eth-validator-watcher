use crate::diesel::RunQueryDsl;
use diesel::{Insertable, PgConnection, QueryResult};
use primitive_types::H256;

use crate::{models::Hash256, schema::execution_blocks};

#[derive(Insertable)]
#[table_name = "execution_blocks"]
pub struct NewExecBlock {
	hash: Hash256,
	number: i64,
	parent_hash: Hash256,
	state_root: Hash256,
	transactions_root: Hash256,
	receipts_root: Hash256,
}

impl NewExecBlock {
	pub fn new(
		hash: H256,
		number: u64,
		parent_hash: H256,
		state_root: H256,
		transactions_root: H256,
		receipts_root: H256,
	) -> NewExecBlock {
		NewExecBlock {
			hash: hash.into(),
			number: number as i64,
			parent_hash: parent_hash.into(),
			state_root: state_root.into(),
			transactions_root: transactions_root.into(),
			receipts_root: receipts_root.into(),
		}
	}

	/// Upser a slot on db
	///
	/// On conflict do nothing
	///
	/// Return the number of affected rows
	pub fn insert_do_nothing(&self, conn: &PgConnection) -> QueryResult<usize> {
		let affected_rows = diesel::insert_into(execution_blocks::table)
			.values(self)
			.on_conflict_do_nothing()
			.execute(conn)?;

		Ok(affected_rows)
	}

	/// Insert a new slot on db
	///
	/// Fail in case of conflict
	pub fn insert(&self, conn: &PgConnection) -> QueryResult<usize> {
		diesel::insert_into(execution_blocks::table).values(self).execute(conn)
	}
}
