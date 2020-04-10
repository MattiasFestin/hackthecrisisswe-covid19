-- Your SQL goes here
CREATE TABLE public.users
(
	"username"					TEXT					NOT NULL,
	"hash"						TEXT					NOT NULL,
	"roles"						JSON					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");


CREATE TABLE public.transactions
(
	"transaction_type_id"		int8					NOT NULL,
	"transaction_direction_id"	int8					NOT NULL,
	-- "lat"						REAL					NOT NULL,
	-- "lng"						REAL					NOT NULL,
	"point"						ST_POINT				NOT NULL,

	"what"						TEXT					NOT NULL,

	"priority"					int8					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

CREATE TABLE public.transactions_constraints
(
	"transactions_id"			uuid					NOT NULL,
	
	"op"						int8					NOT NULL, -- eq, some, all, max, min
	"name"						TEXT					NOT NULL,
	"unit"						TEXT					NOT NULL,
	"value"						REAL					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

-- CREATE TABLE public.constraints
-- (
-- 	"name"						TEXT					NOT NULL,
-- 	"unit_id"					uuid					NOT NULL,
-- 	"constraint_type"			uuid					NOT NULL,

-- 	PRIMARY KEY ("id")

-- ) INHERITS ("base_table");

