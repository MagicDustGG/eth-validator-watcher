use crate::schema::{slots, slots::dsl::slots as dsl_slots};
use diesel::{ExpressionMethods, Identifiable, PgConnection, QueryDsl, Queryable, RunQueryDsl};

#[derive(Queryable, Identifiable, Debug, Clone)]
#[primary_key(spec, height)]
#[table_name = "slots"]
pub struct Slot {
	spec: String,
	height: i64,
	validators_count: Option<i64>,
}

impl Slot {
	pub fn height(&self) -> u64 {
		self.height as u64
	}

	pub fn spec(&self) -> String {
		self.spec.clone()
	}

	pub fn validators_count(&self) -> Option<u64> {
		self.validators_count.map(|c| c as u64)
	}

	/// Return the highest slot stored in database
	pub fn get_highest(conn: &PgConnection) -> Option<Slot> {
		let query = dsl_slots.order(slots::height.desc()).limit(1);
		match query.load::<Slot>(conn) {
			Ok(v) => v.first().cloned(),
			Err(_) => None,
		}
	}

	/// Return the slot at `slot_id`
	pub fn get(conn: &PgConnection, spec: String, height: u64) -> Option<Slot> {
		let query = dsl_slots.find((spec, height as i64));

		query.first(conn).ok()
	}
}
