use crate::schema::{slots, slots::dsl::slots as dsl_slots};
use diesel::{
	ExpressionMethods, Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[primary_key(spec, height)]
#[table_name = "slots"]
pub struct Slot {
	// postgresql doesn't support unsigned types
	// all u64 are stored as i64 and converted back when used
	spec: String,
	height: i64,
	validators_count: Option<i64>,
}

impl Slot {
	/// Return the height of the slot
	pub fn height(&self) -> u64 {
		self.height as u64
	}

	/// Return the spec of the slot
	pub fn spec(&self) -> String {
		self.spec.clone()
	}

	/// Return the validator count of the slot
	pub fn validators_count(&self) -> Option<u64> {
		self.validators_count.map(|c| c as u64)
	}

	/// Return the highest slot from db
	pub fn get_highest(conn: &PgConnection, chain: String) -> QueryResult<Slot> {
		dsl_slots.filter(slots::spec.eq(chain)).order(slots::height.desc()).first(conn)
	}

	/// Return an unique slot from db
	pub fn get(conn: &PgConnection, chain: String, height: u64) -> QueryResult<Slot> {
		dsl_slots.find((chain, height as i64)).first(conn)
	}
}
