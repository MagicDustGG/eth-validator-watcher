mod packed_nft_types;
mod params;

use std::collections::HashMap;

use dotenv::dotenv;
use primitive_types::{H160, H256};
use rocket::{get, launch, routes, serde::json::Json};

use kiln_postgres::{ExecBlock, Slot, Transaction, Validator};
use rocket_sync_db_pools::{database, diesel};

use packed_nft_types::*;
use params::{Hash160, Hash256};

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

#[get("/address/<address>/nfts")]
async fn nfts_by_address(conn: PgConn, address: Hash160) -> Option<Json<PackedNftTypes>> {
	let mut packed_nfts = PackedNftTypes::zero();

	// Transaction related NFTs
	let opt_transactions =
		conn.run(move |c| Transaction::list_from_address(c, address.into())).await.ok();

	if let Some(transactions) = opt_transactions {
		if transactions.len() >= 100 {
			packed_nfts.set_do_100_tansactions()
		}
		if !transactions.is_empty() {
			packed_nfts.set_do_one_transaction();
		}

		let mut deployed_contracts: usize = 0;
		let mut recipients: HashMap<H160, usize> = HashMap::new();
		for t in transactions.into_iter() {
			if t.to().is_none() {
				deployed_contracts += 1;
				continue
			}

			if let Some(to) = t.to() {
				if let Some(c) = recipients.get_mut(&to) {
					*c += 1;
				} else {
					recipients.insert(to, 0);
				}
			}
		}

		if deployed_contracts > 0 {
			packed_nfts.set_deploy_contract();
		}
		if deployed_contracts >= 10 {
			packed_nfts.set_deploy_10_contract();
		}
		if deployed_contracts >= 50 {
			packed_nfts.set_deploy_50_contract();
		}
		if recipients.into_values().filter(|&v| v >= 10).count() >= 10 {
			packed_nfts.set_do_10_transactions_to_10_contracts()
		}
	}

	Some(Json(packed_nfts))
}

#[launch]
fn rocket() -> _ {
	dotenv().ok();
	env_logger::init();

	rocket::build().attach(PgConn::fairing()).mount(
		"/",
		routes![
			slot,
			block,
			transaction,
			validators,
			validators_slashed,
			nfts_by_address
		],
	)
}
