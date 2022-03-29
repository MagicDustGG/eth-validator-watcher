use diesel::{AsChangeset, Insertable};

use super::schema::slots;

#[derive(Insertable, AsChangeset)]
#[table_name = "slots"]
pub struct NewSlot {
	id: i64,
	validators_count: Option<i64>,
}

impl NewSlot {
	pub fn new(id: u64, validators_count: Option<usize>) -> NewSlot {
		NewSlot {
			id: id as i64,
			validators_count: validators_count.map(|c| c as i64),
		}
	}

	pub fn id(&self) -> u64 {
		self.id as u64
	}

	pub fn validators_count(&self) -> Option<u64> {
		self.validators_count.map(|c| c as u64)
	}
}
