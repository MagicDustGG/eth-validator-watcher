use diesel::{
	deserialize::{self, FromSql},
	pg::Pg,
	serialize::{self, IsNull, Output, ToSql},
	sql_types::Binary,
};
use primitive_types::{H160, H256};
use std::io::Write;

// H260

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Clone, Copy, Hash)]
#[sql_type = "Binary"]
pub struct Hash256(H256);

impl From<H256> for Hash256 {
	fn from(hash: H256) -> Self {
		Hash256(hash)
	}
}

impl From<Hash256> for H256 {
	fn from(hash: Hash256) -> Self {
		hash.0
	}
}

impl ToSql<Binary, Pg> for Hash256 {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		out.write_all(self.0.as_bytes())?;
		Ok(IsNull::No)
	}
}

#[allow(clippy::match_single_binding)]
impl FromSql<Binary, Pg> for Hash256 {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match not_none!(bytes) {
			b => Ok(Hash256(H256::from_slice(b))),
		}
	}
}

// H260

#[derive(Debug, PartialEq, Eq, FromSqlRow, AsExpression, Clone, Copy, Hash)]
#[sql_type = "Binary"]
pub struct Hash160(H160);

impl From<H160> for Hash160 {
	fn from(hash: H160) -> Self {
		Hash160(hash)
	}
}

impl From<Hash160> for H160 {
	fn from(hash: Hash160) -> Self {
		hash.0
	}
}

impl ToSql<Binary, Pg> for Hash160 {
	fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		out.write_all(self.0.as_bytes())?;
		Ok(IsNull::No)
	}
}

#[allow(clippy::match_single_binding)]
impl FromSql<Binary, Pg> for Hash160 {
	fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match not_none!(bytes) {
			b => Ok(Hash160(H160::from_slice(b))),
		}
	}
}
