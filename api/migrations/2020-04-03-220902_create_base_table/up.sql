-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION Postgis;

CREATE TABLE public.base_table
(
	"id"					uuid					NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
	"created"				TIMESTAMP				NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"modified"				TIMESTAMP				NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"deleted"				TIMESTAMP				NULL,
	"row_version"			bigint					NOT NULL DEFAULT 1,

	PRIMARY KEY ("id")
);