use rocket::response::Responder;
use rocket_sync_db_pools::diesel;

#[derive(Debug)]
pub enum Error {
	DbPool(diesel::result::Error),
}

impl From<diesel::result::Error> for Error {
	fn from(error: diesel::result::Error) -> Self {
		Self::DbPool(error)
	}
}

impl<'r> Responder<'r, 'static> for Error {
	fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'static> {
		Err(rocket::http::Status::InternalServerError)
	}
}
