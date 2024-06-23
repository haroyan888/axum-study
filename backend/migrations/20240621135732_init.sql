-- Add migration script here
CREATE TABLE todo (
	id					INTEGER PRIMARY KEY,
	title				TEXT NOT NULL,
	description	TEXT,
	completed		BOOLEAN NOT NULL DEFAULT false
);