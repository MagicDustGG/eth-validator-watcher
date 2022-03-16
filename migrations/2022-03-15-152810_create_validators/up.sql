-- Your SQL goes here

CREATE TABLE validators (
    pubkey VARCHAR(98) PRIMARY KEY,
    withdrawal_credentials VARCHAR(66) NOT NULL,
    effective_balance BIGINT UNSIGNED NOT NULL,
    slashed BOOLEAN NOT NULL,
    activation_eligibility_epoch BIGINT UNSIGNED NOT NULL,
    activation_epoch BIGINT UNSIGNED NOT NULL,
    exit_epoch BIGINT UNSIGNED NOT NULL,
    withdrawable_epoch BIGINT UNSIGNED NOT NULL
)
