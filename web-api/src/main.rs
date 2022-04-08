mod errors;
mod packed_nft_types;
mod params;
mod routes;

use dotenv::dotenv;
use rocket::{launch, routes};

use rocket_sync_db_pools::{database, diesel};

pub use errors::Error;

#[database("kiln_pg")]
pub struct PgConn(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build().attach(PgConn::fairing()).mount(
		"/",
		routes![routes::nfts_by_address, routes::list_all_eligible_nft],
	)
}
