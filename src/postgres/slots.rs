use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::{
	error::Error,
	schema::slots::{self, dsl::*},
};

use super::{NewSlot, Slot};

impl Slot {
	/// Return the highest slot stored in database
	pub fn get_highest(conn: &PgConnection) -> Option<Slot> {
		let query = slots.order(id.desc()).limit(1);
		match query.load::<Slot>(conn) {
			Ok(v) => v.first().copied(),
			Err(_) => None,
		}
	}

	/// Return the slot at `slot_id`
	pub fn get(conn: &PgConnection, slot_id: u64) -> Option<Slot> {
		let query = slots.filter(id.eq(slot_id as i64));

		query.first(conn).ok()
	}
}

impl NewSlot {
	/// Upser a slot on db
	///
	/// Return the number of affected rows
	pub fn upsert(&self, conn: &PgConnection) -> Result<usize, Error> {
		let affected_rows = diesel::insert_into(slots::table)
			.values(self)
			.on_conflict_do_nothing()
			.execute(conn)?;

		Ok(affected_rows)
	}

	/// Insert a new slot on db
	///
	/// Fail in case of conflict
	pub fn insert(&self, conn: &PgConnection) -> Result<usize, Error> {
		let affected_row = diesel::insert_into(slots::table).values(self).execute(conn)?;

		Ok(affected_row)
	}
}
