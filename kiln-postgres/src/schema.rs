table! {
	execution_blocks (hash) {
		hash -> Bytea,
		number -> Int8,
		parent_hash -> Bytea,
		state_root -> Bytea,
		transactions_root -> Bytea,
		receipts_root -> Bytea,
	}
}

table! {
	slots (height) {
		height -> Int8,
		block_hash -> Nullable<Bytea>,
		block_number -> Nullable<Int8>,
	}
}

table! {
	transactions (hash) {
		hash -> Bytea,
		block_hash -> Bytea,
		index -> Int8,
		from -> Nullable<Bytea>,
		to -> Nullable<Bytea>,
		input -> Bytea,
	}
}

table! {
	validators (index) {
		index -> Int8,
		balance -> Int8,
		status -> Varchar,
		pubkey -> Varchar,
		withdrawal_credentials -> Bytea,
		effective_balance -> Int8,
		slashed -> Bool,
		activation_eligibility_epoch -> Int8,
		activation_epoch -> Int8,
		exit_epoch -> Int8,
		withdrawable_epoch -> Int8,
	}
}

joinable!(transactions -> execution_blocks (block_hash));

allow_tables_to_appear_in_same_query!(execution_blocks, slots, transactions, validators,);
