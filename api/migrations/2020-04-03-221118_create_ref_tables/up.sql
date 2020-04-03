-- Your SQL goes here
CREATE TABLE public.transaction_types
(
	"id"					integer					NOT NULL DEFAULT 1,
	"name"  				TEXT					NOT NULL,

	PRIMARY KEY ("id")

);

INSERT INTO public.transaction_types (id, name) VALUES
(1, 'Work'),
(2, 'Material'),
(3, 'Transport'),
(4, 'Production');

----------------------------------------------------------------

CREATE TABLE public.transaction_direction
(
	"id"					integer					NOT NULL DEFAULT 1,
	"name"  				TEXT					NOT NULL,

	PRIMARY KEY ("id")

);

INSERT INTO public.transaction_direction (id, name) VALUES
(1, 'Request'),
(2, 'Offer');

----------------------------------------------------------------

CREATE TABLE public.constraint_types
(
	"id"					integer					NOT NULL DEFAULT 1,
	"name"					TEXT					NOT NULL,

	PRIMARY KEY ("id")
);

INSERT INTO public.constraint_types (id, name) VALUES
(1, 'Boolean'),
(2, 'Numeric'),
(2, 'List');

----------------------------------------------------------------