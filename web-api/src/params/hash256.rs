use hex::FromHexError;
use primitive_types::H256;
use rocket::request::FromParam;

pub struct Hash256(H256);

#[derive(Debug)]
pub enum HashParamError {
	/// Invalid string content
	Hex(FromHexError),
	/// Must start with '0x'
	InvalidPrefix,
	/// Must be 66 characters long
	InvalidLength,
}

impl From<FromHexError> for HashParamError {
	fn from(error: FromHexError) -> Self {
		HashParamError::Hex(error)
	}
}

impl<'a> FromParam<'a> for Hash256 {
	type Error = HashParamError;

	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		if &param[..2] != "0x" {
			return Err(HashParamError::InvalidPrefix)
		} else if param.len() != 66 {
			return Err(HashParamError::InvalidLength)
		}

		let vec_repr = hex::decode(&param[2..])?;

		let hash = H256::from_slice(&vec_repr);

		Ok(Hash256(hash))
	}
}

impl From<Hash256> for H256 {
	fn from(hash: Hash256) -> Self {
		hash.0
	}
}
