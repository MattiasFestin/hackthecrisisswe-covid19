-- Postgres script
DROP SCHEMA public CASCADE;

CREATE SCHEMA public
	AUTHORIZATION admin;
	
GRANT ALL ON SCHEMA public TO admin;

GRANT ALL ON SCHEMA public TO PUBLIC;

-- Must be loaded for uuid functions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION Postgis;

CREATE TABLE public.base_table
(
	"id"					uuid					NOT NULL UNIQUE DEFAULT uuid_generate_v4(),
	"created"				TIMESTAMP				NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"modified"				TIMESTAMP				NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"deleted"				TIMESTAMP				NULL,
	"row_version"			integer					NOT NULL DEFAULT 1,

	PRIMARY KEY ("id")
);

----------------------------------------------------------------

CREATE TABLE public.users
(
	"username"				TEXT					NOT NULL,
	"hash"					TEXT					NOT NULL,
	"roles"					JSON					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

----------------------------------------------------------------


CREATE TABLE public.transaction_types
(
	"id"					integer					NOT NULL DEFAULT 1,
	"name"  				TEXT					NOT NULL,

	PRIMARY KEY ("id")

);

INSERT INTO public.transaction_types (id, name) VALUES
(1, 'Work'),
(2, 'Transport'),
(3, 'Production'),
(4, 'Material');

CREATE TABLE  public.transaction_types
(
	"name"  				TEXT					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

----------------------------------------------------------------

CREATE TABLE public.transaction_direction
(
	"name"  				TEXT					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

INSERT INTO public.transaction_direction (id, name) VALUES
('ac20817b-59c1-41f7-bbd1-22da49c48704', 'Request'),
('0f5f757b-5297-4802-ace1-705532409ff7', 'Response');