-- Your SQL goes here
CREATE TABLE public.areas
(
	-- "polygon"		ST_Polygon		NOT NULL,
	"area_code"		TEXT			NOT NULL,
	"parent_area"	uuid			NOT NULL REFERENCES "areas" ("id"),

	PRIMARY KEY ("id")
) INHERITS ("base_table");

CREATE TABLE public.user_areas
(
	"user"			uuid			NOT NULL REFERENCES "users" ("id"),
	"area"			uuid			NOT NULL REFERENCES "areas" ("id"),

	PRIMARY KEY ("id")
) INHERITS ("base_table");