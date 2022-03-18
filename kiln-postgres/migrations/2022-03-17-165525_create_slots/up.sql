-- Your SQL goes here

CREATE TABLE slots (
    spec VARCHAR(10) NOT NULL REFERENCES specs(name) ON DELETE CASCADE,
    height BIGINT,
    validators_count BIGINT,
    PRIMARY KEY (spec, height)
)
