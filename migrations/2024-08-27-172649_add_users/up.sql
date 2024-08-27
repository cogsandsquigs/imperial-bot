-- Your SQL goes here

CREATE TYPE user_state AS ENUM ('unverified', 'querying_email', 'querying_otp', 'verified');

CREATE TABLE users (
	id				bigint PRIMARY KEY,
	imperial_email	varchar,
	state			user_state NOT NULL DEFAULT 'unverified',
	otps			integer[] NOT NULL DEFAULT '{}' check (array_position(otps, null) is null)
);

