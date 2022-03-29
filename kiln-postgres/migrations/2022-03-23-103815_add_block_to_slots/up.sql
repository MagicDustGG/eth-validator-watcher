-- Your SQL goes here

ALTER TABLE slots
ADD COLUMN block_hash VARCHAR(256),
ADD COLUMN block_number BIGINT;
