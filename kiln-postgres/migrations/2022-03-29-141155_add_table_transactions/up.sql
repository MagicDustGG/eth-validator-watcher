-- Your SQL goes here

CREATE TABLE transactions(
    "hash" BYTEA PRIMARY KEY,
    "block_hash" BYTEA NOT NULL REFERENCES execution_blocks(hash),
    "index" BIGINT NOT NULL,
    "from" BYTEA,
    "to" BYTEA,
    input BYTEA NOT NULL,
    "value" BYTEA NOT NULL,
    "status" boolean
);

CREATE INDEX block_idx
ON transactions(block_hash);
