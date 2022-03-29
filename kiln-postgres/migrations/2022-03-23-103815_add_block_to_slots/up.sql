-- Your SQL goes here

ALTER TABLE slots
ADD COLUMN block_hash BYTEA,
ADD COLUMN block_number BIGINT;
