-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
	id SERIAL,
	name VARCHAR(50) NOT NULL,
	added_at TIMESTAMPTZ NOT NULL,
	UNIQUE (name)
)
