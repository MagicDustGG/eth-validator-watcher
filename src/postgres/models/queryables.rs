use diesel::Queryable;

#[allow(dead_code)]
#[derive(Queryable, Debug, Clone, Copy)]
pub struct Slot {
	id: i64,
	validators_count: Option<i64>,
}

impl Slot {
	pub fn id(&self) -> u64 {
		self.id as u64
	}
}
