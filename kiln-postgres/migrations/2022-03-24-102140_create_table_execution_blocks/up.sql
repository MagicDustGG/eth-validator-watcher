-- Your SQL goes here

CREATE TABLE execution_blocks (
    hash BYTEA PRIMARY KEY,
    number BIGINT NOT NULL,
    parent_hash BYTEA NOT NULL,
    state_root BYTEA NOT NULL,
    transactions_root BYTEA NOT NULL,
    receipts_root BYTEA NOT NULL
);
