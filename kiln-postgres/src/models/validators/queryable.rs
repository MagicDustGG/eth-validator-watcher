use diesel::{ExpressionMethods, Identifiable, PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use primitive_types::H256;
use serde::{Deserialize, Serialize};

use crate::{
	models::Hash256,
	schema::{validators, validators::dsl::validators as dsl_validators},
};

#[derive(Queryable, Identifiable)]
#[primary_key(index)]
#[table_name = "validators"]
struct DbValidator {
	index: i64,
	balance: i64,
	status: String,
	pubkey: String,
	withdrawal_credentials: Hash256,
	effective_balance: i64,
	slashed: bool,
	activation_eligibility_epoch: i64,
	activation_epoch: i64,
	exit_epoch: i64,
	withdrawable_epoch: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Validator {
	index: u64,
	balance: u64,
	status: String,
	pubkey: String,
	withdrawal_credentials: H256,
	effective_balance: u64,
	slashed: bool,
	activation_eligibility_epoch: u64,
	activation_epoch: u64,
	exit_epoch: u64,
	withdrawable_epoch: u64,
}

impl From<DbValidator> for Validator {
	fn from(db_validator: DbValidator) -> Self {
		Validator {
			index: db_validator.index as u64,
			balance: db_validator.balance as u64,
			status: db_validator.status,
			pubkey: db_validator.pubkey,
			withdrawal_credentials: db_validator.withdrawal_credentials.into(),
			effective_balance: db_validator.effective_balance as u64,
			slashed: db_validator.slashed,
			activation_eligibility_epoch: db_validator.activation_eligibility_epoch as u64,
			activation_epoch: db_validator.activation_epoch as u64,
			exit_epoch: db_validator.exit_epoch as u64,
			withdrawable_epoch: db_validator.withdrawable_epoch as u64,
		}
	}
}

impl Validator {
	/// Return a list of all validators `withdrawal_credentials`
	pub fn list_credentials(conn: &PgConnection) -> QueryResult<Vec<H256>> {
		let credentials: Vec<Hash256> =
			dsl_validators.select(validators::withdrawal_credentials).load(conn)?;

		Ok(credentials.into_iter().map(|v| v.into()).collect())
	}

	/// Return a list of all slashed validators `withdrawal_credentials`
	pub fn list_slashed_credentials(conn: &PgConnection) -> QueryResult<Vec<H256>> {
		let slashed: Vec<Hash256> = dsl_validators
			.filter(validators::slashed.eq(true))
			.select(validators::withdrawal_credentials)
			.load(conn)?;

		Ok(slashed.into_iter().map(|v| v.into()).collect())
	}
}
