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
		validators_count -> Int8,
		block_hash -> Nullable<Bytea>,
		block_number -> Nullable<Int8>,
	}
}

allow_tables_to_appear_in_same_query!(execution_blocks, slots,);
