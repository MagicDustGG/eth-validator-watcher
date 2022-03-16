use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Validator {
	pub pubkey: String,
	pub withdrawal_credentials: String,
	pub effective_balance: u64,
	pub slashed: bool,
	pub activation_eligibility_epoch: u64,
	pub activation_epoch: u64,
	pub exit_epoch: u64,
	pub withdrawable_epoch: u64,
}
