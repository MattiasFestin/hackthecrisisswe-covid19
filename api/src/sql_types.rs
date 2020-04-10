use std::convert::{From, Into};
use std::io::prelude::*;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
// use diesel::*;

use postgis::ewkb::{Point};
use postgis::ewkb::{EwkbRead, EwkbWrite};
use postgis::ewkb::AsEwkbPoint;

#[derive(Debug)]
#[postgres(type_name = "st_point")]
pub struct PgPoint(pub Point);

impl ToSql<PgPoint, Pg> for PgPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		let point = (*self).0;

		match out.write_all(point.as_ewkb().to_hex_ewkb().as_bytes()) {
			Ok(_data) => Ok(IsNull::No),
			_ => Err("Could not write point to database".into()),
		}
    }
}

impl FromSql<PgPoint, Pg> for PgPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		match bytes {
			Some(data) => {
				let mut to_write = data.clone();
				Ok(PgPoint(Point::read_ewkb(&mut to_write).unwrap()))
			},
			None =>  Err("Unrecognized data".into()),
		}
    }
}

impl From<PgPoint> for Point {
	fn from(p: PgPoint) -> Self {
		p.0
	}
}


impl From<Point> for PgPoint {
	fn from(p: Point) -> Self {
		PgPoint(p)
	}
}