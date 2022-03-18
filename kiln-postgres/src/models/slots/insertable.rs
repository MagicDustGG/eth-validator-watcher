use crate::diesel::RunQueryDsl;
use diesel::{Insertable, PgConnection, QueryResult};

use crate::schema::slots;

#[derive(Insertable)]
#[table_name = "slots"]
pub struct NewSlot {
	spec: String,
	height: i64,
	validators_count: Option<i64>,
}

impl NewSlot {
	pub fn new(spec: String, height: u64, validators_count: Option<usize>) -> NewSlot {
		NewSlot {
			spec,
			height: height as i64,
			validators_count: validators_count.map(|c| c as i64),
		}
	}

	pub fn height(&self) -> u64 {
		self.height as u64
	}

	pub fn validators_count(&self) -> Option<u64> {
		self.validators_count.map(|c| c as u64)
	}

	/// Upser a slot on db
	///
	/// Return the number of affected rows
	pub fn upsert(&self, conn: &PgConnection) -> QueryResult<usize> {
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
