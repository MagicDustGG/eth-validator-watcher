use eth2::types::ValidatorData;

use crate::schema::validators;

#[derive(Insertable)]
#[table_name = "validators"]
pub struct NewValidator {
	pub pubkey: String,
	pub withdrawal_credentials: String,
	pub effective_balance: u64,
	pub slashed: bool,
	pub activation_eligibility_epoch: u64,
	pub activation_epoch: u64,
	pub exit_epoch: u64,
	pub withdrawable_epoch: u64,
}

impl From<ValidatorData> for NewValidator {
	fn from(data: ValidatorData) -> Self {
		NewValidator {
			pubkey: data.validator.pubkey.to_string(),
			withdrawal_credentials: data.validator.withdrawal_credentials.to_string(),
			effective_balance: data.validator.effective_balance,
			slashed: data.validator.slashed,
			activation_eligibility_epoch: data.validator.activation_eligibility_epoch.into(),
			activation_epoch: data.validator.activation_epoch.into(),
			exit_epoch: data.validator.exit_epoch.into(),
			withdrawable_epoch: data.validator.withdrawable_epoch.into(),
		}
	}
}
