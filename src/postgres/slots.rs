use diesel::{pg::upsert::excluded, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::error::Error;

use super::{
	models::schema::slots::{self, dsl::*},
	NewSlot, Slot,
};

impl Slot {
	pub fn get_highest(conn: &PgConnection) -> Option<Slot> {
		let query = slots.order(id.desc()).limit(1);
		match query.load::<Slot>(conn) {
			Ok(v) => v.first().copied(),
			Err(_) => None,
		}
	}

	pub fn get(conn: &PgConnection, slot_id: u64) -> Option<Slot> {
		let query = slots.filter(id.eq(slot_id as i64));

		query.first(conn).ok()
	}
}

impl NewSlot {
	pub fn batch_upsert(conn: &PgConnection, new_slot: Vec<NewSlot>) -> usize {
		diesel::insert_into(slots::table)
			.values(&new_slot)
			.on_conflict(slots::id)
			.do_update()
			.set(slots::validators_count.eq(excluded(slots::validators_count)))
			.execute(conn)
			.expect("Error saving new slot")
	}

	pub fn upsert(&self, conn: &PgConnection) -> Result<usize, Error> {
		let affected_row = diesel::insert_into(slots::table)
			.values(self)
			.on_conflict_do_nothing()
			.execute(conn)?;

		Ok(affected_row)
	}

	pub fn insert(&self, conn: &PgConnection) -> Result<usize, Error> {
		let affected_row = diesel::insert_into(slots::table).values(self).execute(conn)?;

		Ok(affected_row)
	}
}
