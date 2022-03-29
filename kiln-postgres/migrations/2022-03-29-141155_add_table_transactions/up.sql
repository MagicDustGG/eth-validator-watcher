-- Your SQL goes here

CREATE TABLE transactions(
    "hash" BYTEA PRIMARY KEY,
    "block_hash" BYTEA NOT NULL,
    "index" BIGINT NOT NULL,
    "from" BYTEA,
    "to" BYTEA,
    CONSTRAINT fk_block_hash FOREIGN KEY(block_hash) REFERENCES execution_blocks(hash)
);

CREATE INDEX block_idx
ON transactions(block_hash);
