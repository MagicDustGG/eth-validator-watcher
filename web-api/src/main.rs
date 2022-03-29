mod params;

use dotenv::dotenv;
use rocket::{get, launch, routes, serde::json::Json};

use kiln_postgres::{ExecBlock, Slot, Transaction};
use rocket_sync_db_pools::{database, diesel};

use params::Hash256;

#[database("kiln_pg")]
struct PgConn(diesel::PgConnection);

#[get("/slot/<height>")]
async fn slot(conn: PgConn, height: u64) -> Option<Json<Slot>> {
	conn.run(move |c| Slot::get(c, height)).await.map(Json).ok()
}

#[get("/block/<number>")]
async fn block(conn: PgConn, number: u64) -> Option<Json<ExecBlock>> {
	conn.run(move |c| ExecBlock::get(c, number)).await.map(Json).ok()
}

#[get("/transaction/<hash>")]
async fn transaction(conn: PgConn, hash: Hash256) -> Option<Json<Transaction>> {
	conn.run(move |c| Transaction::get(c, hash.into())).await.map(Json).ok()
}

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build()
		.attach(PgConn::fairing())
		.mount("/", routes![slot, block, transaction])
}
