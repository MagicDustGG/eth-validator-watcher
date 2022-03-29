use dotenv::dotenv;
use rocket::{get, launch, routes, serde::json::Json};

use kiln_postgres::Slot;
use rocket_sync_db_pools::{database, diesel};

#[database("kiln_pg")]
struct PgConn(diesel::PgConnection);

#[get("/<chain>/slot/<height>")]
async fn slot(conn: PgConn, chain: String, height: u64) -> Option<Json<Slot>> {
	conn.run(move |c| Slot::get(c, chain, height)).await.map(Json).ok()
}

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build().attach(PgConn::fairing()).mount("/", routes![slot])
}
