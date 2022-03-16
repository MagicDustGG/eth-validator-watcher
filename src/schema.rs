table! {
    validators (pubkey) {
        pubkey -> Varchar,
        withdrawal_credentials -> Varchar,
        effective_balance -> Unsigned<Bigint>,
        slashed -> Bool,
        activation_eligibility_epoch -> Unsigned<Bigint>,
        activation_epoch -> Unsigned<Bigint>,
        exit_epoch -> Unsigned<Bigint>,
        withdrawable_epoch -> Unsigned<Bigint>,
    }
}
