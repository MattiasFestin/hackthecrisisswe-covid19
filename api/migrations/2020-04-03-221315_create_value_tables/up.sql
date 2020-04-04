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
	"lat"						REAL					NOT NULL,
	"lng"						REAL					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

CREATE TABLE public.transactions_constraints
(
	"transactions_id"			uuid					NOT NULL,
	"constraints_id"			uuid					NOT NULL,
	"operation"					JSON					NULL, -- eq, some, all, max, min

	PRIMARY KEY ("id")

) INHERITS ("base_table");

CREATE TABLE public.constraints
(
	"name"						TEXT					NOT NULL,
	"unit_id"					uuid					NOT NULL,
	"constraint_type"			uuid					NOT NULL,

	PRIMARY KEY ("id")

) INHERITS ("base_table");

