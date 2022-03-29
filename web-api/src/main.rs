use dotenv::dotenv;
use rocket::{get, launch, routes, serde::json::Json};

use kiln_postgres::{ExecBlock, Slot};
use rocket_sync_db_pools::{database, diesel};

#[database("kiln_pg")]
struct PgConn(diesel::PgConnection);

#[get("/slot/<height>")]
async fn slot(conn: PgConn, height: u64) -> Option<Json<Slot>> {
	conn.run(move |c| Slot::get(c, height)).await.map(Json).ok()
}

#[get("/block/<height>")]
async fn block(conn: PgConn, height: u64) -> Option<Json<ExecBlock>> {
	conn.run(move |c| ExecBlock::get(c, height)).await.map(Json).ok()
}

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build().attach(PgConn::fairing()).mount("/", routes![slot, block])
}
