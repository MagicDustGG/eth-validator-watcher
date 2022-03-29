use diesel::{Insertable, PgConnection, QueryResult, RunQueryDsl};

use crate::schema::specs;

#[derive(Insertable)]
#[table_name = "specs"]
pub struct NewSpec {
	name: String,
	preset_base: String,
}

impl NewSpec {
	pub fn new(name: &str, preset_base: &str) -> NewSpec {
		NewSpec {
			name: name.to_string(),
			preset_base: preset_base.to_string(),
		}
	}

	/// Upser a spec on db
	///
	/// Return the number of affected rows
	pub fn upsert(&self, conn: &PgConnection) -> QueryResult<usize> {
		let affected_rows = diesel::insert_into(specs::table)
			.values(self)
			.on_conflict_do_nothing()
			.execute(conn)?;

		Ok(affected_rows)
	}
}
