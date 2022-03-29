-- This file should undo anything in `up.sql`

ALTER TABLE slots
DROP COLUMN block_hash,
DROP COLUMN block_number;
