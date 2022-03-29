-- Your SQL goes here

CREATE TABLE slots (
    height BIGINT PRIMARY KEY,
    validators_count BIGINT NOT NULL,
    block_hash BYTEA,
    block_number BIGINT
)
