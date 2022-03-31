mod params;

use dotenv::dotenv;
use primitive_types::H256;
use rocket::{get, launch, routes, serde::json::Json};

use kiln_postgres::{ExecBlock, Slot, Transaction, Validator};
use rocket_sync_db_pools::{database, diesel};

use params::Hash256;

#[database("kiln_pg")]
struct PgConn(diesel::PgConnection);

/// Return a consensus layer slot by `height`
#[get("/slot/<height>")]
async fn slot(conn: PgConn, height: u64) -> Option<Json<Slot>> {
	conn.run(move |c| Slot::get(c, height)).await.map(Json).ok()
}

/// Return an execution layer block by `height`
#[get("/block/<number>")]
async fn block(conn: PgConn, number: u64) -> Option<Json<ExecBlock>> {
	conn.run(move |c| ExecBlock::get(c, number)).await.map(Json).ok()
}

/// Return an execution block transaction by `hash`
#[get("/transaction/<hash>")]
async fn transaction(conn: PgConn, hash: Hash256) -> Option<Json<Transaction>> {
	conn.run(move |c| Transaction::get(c, hash.into())).await.map(Json).ok()
}

/// Return the validators IDs
///
/// The returned values are hashs of the `withdrawal_credentials` associated to a validator
#[get("/validators")]
async fn validators(conn: PgConn) -> Option<Json<Vec<H256>>> {
	conn.run(move |c| Validator::list_credentials(c)).await.map(Json).ok()
}

/// Return the slashed validators IDs
///
/// The returned values are hashs of the `withdrawal_credentials` associated to a validator
#[get("/validators/slashed")]
async fn validators_slashed(conn: PgConn) -> Option<Json<Vec<H256>>> {
	conn.run(move |c| Validator::list_slashed_credentials(c)).await.map(Json).ok()
}

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build().attach(PgConn::fairing()).mount(
		"/",
		routes![slot, block, transaction, validators, validators_slashed],
	)
}
