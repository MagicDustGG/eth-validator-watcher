use diesel::{
	deserialize,
	pg::Pg,
	serialize::{self, IsNull, Output},
	sql_types::Binary,
	types::{FromSql, ToSql},
};
use primitive_types::H256;
use std::io::Write;

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
