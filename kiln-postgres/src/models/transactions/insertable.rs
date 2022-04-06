use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use primitive_types::{H160, H256, U256};

use crate::{
	models::{Hash160, Hash256},
	schema::transactions,
};

#[derive(Insertable, Identifiable)]
#[primary_key(hash)]
#[table_name = "transactions"]
pub struct NewTransaction {
	hash: Hash256,
	block_hash: Hash256,
	index: i64,
	from: Option<Hash160>,
	to: Option<Hash160>,
	input: Vec<u8>,
	value: Vec<u8>,
}

impl NewTransaction {
	/// Return a new insertable Transaction
	pub fn new(
		hash: H256,
		block_hash: H256,
		index: u64,
		from: Option<H160>,
		to: Option<H160>,
		input: Vec<u8>,
		value: U256,
	) -> NewTransaction {
		NewTransaction {
			hash: hash.into(),
			block_hash: block_hash.into(),
			index: index as i64,
			from: from.map(|f| f.into()),
			to: to.map(|t| t.into()),
			input,
			value: u256_to_vec_u8(value),
		}
	}

	/// Insert a new transaction on db
	///
	/// Fail in case of conflict
	pub fn insert(&self, conn: &PgConnection) -> QueryResult<usize> {
		diesel::insert_into(transactions::table).values(self).execute(conn)
	}

	/// Set a transaction status
	pub fn set_status(conn: &PgConnection, hash: H256, status: bool) -> QueryResult<usize> {
		let hash: Hash256 = hash.into();
		diesel::update(transactions::dsl::transactions.find(hash))
			.set(transactions::status.eq(Some(status)))
			.execute(conn)
	}
}

pub struct NewTransactions(Vec<NewTransaction>);

impl NewTransactions {
	pub fn batch_insert(&self, conn: &PgConnection) -> QueryResult<usize> {
		diesel::insert_into(transactions::table).values(&self.0).execute(conn)
	}

	pub fn new(transactions: Vec<NewTransaction>) -> Self {
		Self(transactions)
	}
}

impl FromIterator<NewTransaction> for NewTransactions {
	fn from_iter<T: IntoIterator<Item = NewTransaction>>(iter: T) -> Self {
		let mut transactions = vec![];
		for t in iter {
			transactions.push(t);
		}
		NewTransactions(transactions)
	}
}

// U256 internal representation is `&[u64; 4]`.
// We need it as `[u8]` to be stored as BYTEA in db.
// Keep the pointed memory untouched but reorganise `len`, `capacity` and type of the reference.
fn u256_to_vec_u8(value: U256) -> Vec<u8> {
	let value_as_vec_u64: Vec<u64> = value.0.to_vec();
	let ratio = std::mem::size_of::<u64>() / std::mem::size_of::<u8>();

	let length = value_as_vec_u64.len() * ratio;
	let capacity = value_as_vec_u64.capacity() * ratio;
	let ptr = value_as_vec_u64.as_ptr() as *mut u8;

	std::mem::forget(value_as_vec_u64);

	// Safe because we forget the original reference before creating the new one
	// and because we adjusted `lenght` and `capacity` proportionaly.
	unsafe { Vec::from_raw_parts(ptr, length, capacity) }
}
