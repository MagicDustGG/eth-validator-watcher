table! {
	slots (spec, height) {
		spec -> Varchar,
		height -> Int8,
		validators_count -> Nullable<Int8>,
		block_hash -> Nullable<Varchar>,
		block_number -> Nullable<Int8>,
	}
}

table! {
	specs (name) {
		name -> Varchar,
		preset_base -> Varchar,
	}
}

joinable!(slots -> specs (spec));

allow_tables_to_appear_in_same_query!(slots, specs,);
