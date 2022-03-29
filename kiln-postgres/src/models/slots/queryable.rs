use crate::{
	schema::{slots, slots::dsl::slots as dsl_slots},
	types::Hash256,
};
use diesel::{
	ExpressionMethods, Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl,
};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable)]
#[primary_key(height)]
#[table_name = "slots"]
struct DbSlot {
	// postgresql doesn't support unsigned types
	// all u64 are stored as i64 and converted back when used
	height: i64,
	validators_count: i64,
	block_hash: Option<Hash256>,
	block_number: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Slot {
	height: u64,
	validators_count: u64,
	block_hash: Option<H256>,
	block_number: Option<u64>,
}

impl From<DbSlot> for Slot {
	fn from(db_slot: DbSlot) -> Self {
		Slot {
			height: db_slot.height as u64,
			validators_count: db_slot.validators_count as u64,
			block_hash: db_slot.block_hash.map(|h| h.into()),
			block_number: db_slot.block_number.map(|n| n as u64),
		}
	}
}

impl Slot {
	/// Return the height of the slot
	pub fn height(&self) -> u64 {
		self.height
	}

	/// Return the validator count of the slot
	pub fn validators_count(&self) -> u64 {
		self.validators_count
	}

	/// Return the hash of the slot's execution block
	pub fn block_hash(&self) -> Option<H256> {
		self.block_hash
	}

	/// Return the number of the slot's execution block
	pub fn block_number(&self) -> Option<u64> {
		self.block_number
	}

	/// Return the highest slot from db
	pub fn get_highest(conn: &PgConnection) -> QueryResult<Slot> {
		let slot = dsl_slots.order(slots::height.desc()).first::<DbSlot>(conn)?;

		Ok(slot.into())
	}

	/// Return an unique slot from db
	pub fn get(conn: &PgConnection, height: u64) -> QueryResult<Slot> {
		let slot = dsl_slots.find(height as i64).first::<DbSlot>(conn)?;

		Ok(slot.into())
	}
}
