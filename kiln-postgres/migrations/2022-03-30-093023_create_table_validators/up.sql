-- Your SQL goes here

CREATE TABLE validators (
    "index" BIGINT PRIMARY KEY,
    balance BIGINT NOT NULL,
    "status" VARCHAR NOT NULL,
    pubkey VARCHAR NOT NULL,
    withdrawal_credentials BYTEA NOT NULL,
    effective_balance BIGINT NOT NULL,
    slashed BOOLEAN NOT NULL,
    activation_eligibility_epoch BIGINT NOT NULL,
    activation_epoch BIGINT NOT NULL,
    exit_epoch BIGINT NOT NULL,
    withdrawable_epoch BIGINT NOT NULL,
    deposit_transaction BYTEA REFERENCES transactions(hash)
);
