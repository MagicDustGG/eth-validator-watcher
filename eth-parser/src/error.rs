use std::{env::VarError, fmt::Display};

use sensitive_url::SensitiveError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum Error {
	Eth2(eth2::Error),
	Var(VarError),
	Sensitive(SensitiveError),
	Join(JoinError),
	Diesel(diesel::result::Error),
	/// Chain preset not supported
	InvalidChainPreset(String),
	/// Config name is missing from chain config
	MissingChainName,
}

impl From<eth2::Error> for Error {
	fn from(error: eth2::Error) -> Self {
		Error::Eth2(error)
	}
}

impl From<VarError> for Error {
	fn from(error: VarError) -> Self {
		Error::Var(error)
	}
}

impl From<SensitiveError> for Error {
	fn from(error: SensitiveError) -> Self {
		Error::Sensitive(error)
	}
}

impl From<JoinError> for Error {
	fn from(error: JoinError) -> Self {
		Error::Join(error)
	}
}

impl From<diesel::result::Error> for Error {
	fn from(error: diesel::result::Error) -> Self {
		Error::Diesel(error)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidChainPreset(p) => write!(
				f,
				"'{}' preset not supported. Only 'mainnet' is supported",
				p
			),
			Self::MissingChainName => write!(f, "Invalid config. 'config_name' is required."),
			_ => write!(f, "{:?}", self),
		}
	}
}
