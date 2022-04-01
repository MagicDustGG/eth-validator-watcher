use hex::FromHexError;
use primitive_types::H160;
use rocket::request::FromParam;

pub struct Hash160(H160);

#[derive(Debug)]
pub enum HashParamError {
	/// Invalid string content
	Hex(FromHexError),
	/// Must start with '0x'
	InvalidPrefix,
	/// Must be 42 characters long
	InvalidLength,
}

impl From<FromHexError> for HashParamError {
	fn from(error: FromHexError) -> Self {
		HashParamError::Hex(error)
	}
}

impl<'a> FromParam<'a> for Hash160 {
	type Error = HashParamError;

	fn from_param(param: &'a str) -> Result<Self, Self::Error> {
		if &param[..2] != "0x" {
			return Err(HashParamError::InvalidPrefix)
		} else if param.len() != 42 {
			return Err(HashParamError::InvalidLength)
		}

		let vec_repr = hex::decode(&param[2..])?;

		let hash = H160::from_slice(&vec_repr);

		Ok(Hash160(hash))
	}
}

impl From<Hash160> for H160 {
	fn from(hash: Hash160) -> Self {
		hash.0
	}
}
