use std::convert::{From, Into};
use std::io::prelude::*;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
// use diesel::*;

use postgis::ewkb::{Point};
use postgis::ewkb::{EwkbRead, EwkbWrite};
use postgis::ewkb::AsEwkbPoint;

 #[derive(SqlType)]
#[postgres(type_name = "geography")]
pub struct Geography;

#[derive(Debug, Copy, Clone, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "Geography"]
pub struct PGPoint(pub Point);

impl ToSql<Geography, Pg> for PGPoint {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
		use postgis::ewkb::{AsEwkbPoint, EwkbWrite};
		Point::from(*self).as_ewkb().write_ewkb(out)?;
		Ok(IsNull::No)
    }
}

impl FromSql<Geography, Pg> for PGPoint {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
		use postgis::ewkb::EwkbRead;
		use std::io::Cursor;
		let bytes = not_none!(bytes);
		let mut rdr = Cursor::new(bytes);
		Ok(Point::read_ewkb(&mut rdr)?.into())
    }
}

impl From<PGPoint> for Point {
	fn from(p: PGPoint) -> Self {
		p.0
	}
}


impl From<Point> for PGPoint {
	fn from(p: Point) -> Self {
		PGPoint(p)
	}
}