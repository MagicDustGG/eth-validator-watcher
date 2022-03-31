use diesel::{Insertable, PgConnection, QueryResult, RunQueryDsl};
use primitive_types::H256;

use crate::{models::Hash256, schema::slots};

/// Representation of a row to be inserted
#[derive(Insertable)]
#[table_name = "slots"]
pub struct NewSlot {
	// postgresql doesn't support unsigned types
	// all u64 are stored as i64 and converted back when used
	height: i64,
	block_hash: Option<Hash256>,
	block_number: Option<i64>,
}

impl NewSlot {
	/// Return a new insertable slot
	pub fn new(height: u64, block_hash: Option<H256>, block_number: Option<u64>) -> NewSlot {
		NewSlot {
			height: height as i64,
			block_hash: block_hash.map(|h| h.into()),
			block_number: block_number.map(|n| n as i64),
		}
	}

	/// Upser a slot on db
	///
	/// On conflict do nothing
	///
	/// Return the number of affected rows
	pub fn insert_do_nothing(&self, conn: &PgConnection) -> QueryResult<usize> {
		let affected_rows = diesel::insert_into(slots::table)
			.values(self)
			.on_conflict_do_nothing()
			.execute(conn)?;

		Ok(affected_rows)
	}

	/// Insert a new slot on db
	///
	/// Fail in case of conflict
	pub fn insert(&self, conn: &PgConnection) -> QueryResult<usize> {
		diesel::insert_into(slots::table).values(self).execute(conn)
	}
}
