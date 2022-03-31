use diesel::{
	pg::upsert::excluded, ExpressionMethods, Insertable, PgConnection, QueryResult, RunQueryDsl,
};
use eth2::types::ValidatorData;

use crate::{models::Hash256, schema::validators};

#[derive(Insertable)]
#[table_name = "validators"]
pub struct NewValidator {
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

impl From<ValidatorData> for NewValidator {
	fn from(data: ValidatorData) -> Self {
		NewValidator {
			index: data.index as i64,
			balance: data.balance as i64,
			status: data.status.to_string(),
			pubkey: data.validator.pubkey.to_string(),
			withdrawal_credentials: data.validator.withdrawal_credentials.into(),
			effective_balance: data.validator.effective_balance as i64,
			slashed: data.validator.slashed,
			activation_eligibility_epoch: data.validator.activation_eligibility_epoch.as_u64()
				as i64,
			activation_epoch: data.validator.activation_epoch.as_u64() as i64,
			exit_epoch: data.validator.exit_epoch.as_u64() as i64,
			withdrawable_epoch: data.validator.withdrawable_epoch.as_u64() as i64,
		}
	}
}

/// An wrapper around an array fo validators
pub struct NewValidators(Vec<NewValidator>);

impl NewValidators {
	/// Upsert an array of validators in db
	///
	/// # Updated fields
	/// `balance`, `status`, `withdrawal_credentials`, `effective_balance`, `slashed`
	pub fn batch_upsert(&self, conn: &PgConnection) -> QueryResult<()> {
		for chunk in self.0.chunks(1000) {
			diesel::insert_into(validators::table)
				.values(chunk)
				.on_conflict(validators::index)
				.do_update()
				.set((
					validators::balance.eq(excluded(validators::balance)),
					validators::status.eq(excluded(validators::status)),
					validators::withdrawal_credentials
						.eq(excluded(validators::withdrawal_credentials)),
					validators::effective_balance.eq(excluded(validators::effective_balance)),
					validators::slashed.eq(excluded(validators::slashed)),
				))
				.execute(conn)?;
		}

		Ok(())
	}
}

impl FromIterator<NewValidator> for NewValidators {
	fn from_iter<T: IntoIterator<Item = NewValidator>>(iter: T) -> Self {
		let mut validators = vec![];
		for t in iter {
			validators.push(t);
		}
		NewValidators(validators)
	}
}
