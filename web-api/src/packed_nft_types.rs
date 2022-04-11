use paste::paste;
use primitive_types::U256;
use serde::Serialize;

macro_rules! create_nft_getter_and_setter {
	($nft_name:ident, $position: literal) => {
		paste! {
			#[allow(dead_code)]
			pub fn [<get_ $nft_name>](&self) -> bool {
				self.0.bit($position)
			}

			pub fn [<set_ $nft_name>](&mut self) {
				self.0 = self.0 | U256::one() << $position;
			}
		}
	};
}

#[derive(Serialize)]
pub struct PackedNftTypes(U256);

impl PackedNftTypes {
	create_nft_getter_and_setter!(do_one_transaction, 0);

	create_nft_getter_and_setter!(do_100_tansactions, 1);

	create_nft_getter_and_setter!(deploy_contract, 2);

	create_nft_getter_and_setter!(deploy_10_contract, 3);

	create_nft_getter_and_setter!(deploy_50_contract, 4);

	create_nft_getter_and_setter!(do_10_transactions_to_10_contracts, 5);

	create_nft_getter_and_setter!(become_validator, 6);

	create_nft_getter_and_setter!(slashed_validator, 7);

	pub fn zero() -> Self {
		PackedNftTypes(U256::zero())
	}
}
