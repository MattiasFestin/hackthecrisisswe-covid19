use std::time::SystemTime;
use serde::{Serialize,Deserialize};
// use rocket::data::FromDataSimple;
// use crate::geoencoding::Address;

#[derive(Serialize, Deserialize)]
pub struct VMTransaction {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub transaction_direction_id: i64,
    pub transaction_type_id: i64,

    pub priority: i64,

    pub what: String,
    pub r#where: Option<crate::geoencoding::Address>,

    pub lat: f64,
    pub lng: f64,

    pub constraints: Vec<VMConstraint>,
}

#[derive(Serialize, Deserialize)]
pub struct VMInsertTransaction {
    // pub id: uuid::Uuid,
    // pub created: SystemTime,
    // pub modified: SystemTime,
    // pub deleted: Option<SystemTime>,
    // pub row_version: i64,

    pub transaction_direction_id: i64,
    pub transaction_type_id: i64,

    pub priority: i64,

    pub what: String,

    pub lat: f64,
    pub lng: f64,

    pub constraints: Vec<VMConstraint>,
}

#[derive(Serialize, Deserialize)]
pub struct VMUpdateTransaction {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub row_version: i64,

    pub transaction_direction_id: i64,
    pub transaction_type_id: i64,

    pub priority: i64,

    pub what: String,

    pub lat: f64,
    pub lng: f64,

    pub constraints: Vec<VMConstraint>,
}

// impl FromDataSimple for VMTransaction {
//     type Error = String;

//     fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
//         // // Ensure the content type is correct before opening the data.
//         // let person_ct = ContentType::new("application", "x-person");
//         // if req.content_type() != Some(&person_ct) {
//         //     return Outcome::Forward(data);
//         // }

//         // // Read the data into a String.
//         // let mut string = String::new();
//         // if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
//         //     return Failure((Status::InternalServerError, format!("{:?}", e)));
//         // }

//         // // Split the string into two pieces at ':'.
//         // let (name, age) = match string.find(':') {
//         //     Some(i) => (string[..i].to_string(), &string[(i + 1)..]),
//         //     None => return Failure((Status::UnprocessableEntity, "':'".into()))
//         // };

//         // // Parse the age.
//         // let age: u16 = match age.parse() {
//         //     Ok(age) => age,
//         //     Err(_) => return Failure((Status::UnprocessableEntity, "Age".into()))
//         // };

//         // Return successfully.
//         Success(Person { name, age })
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct VMConstraint {
    pub id: uuid::Uuid,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub deleted: Option<SystemTime>,
    pub row_version: i64,

    pub name: String,
    pub unit: String,
    pub op: i64,
    pub value: f32
}

#[derive(Serialize, Deserialize)]
pub enum TransactionDirEnum {
    Request,
    Offer
}

#[derive(Serialize, Deserialize)]
pub enum TransactionTypesEnum {
    Work,
    Material,
    Transport,
    Production
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintTypeEnum {
    Boolean,
    List,
    Numeric
}

#[derive(Serialize, Deserialize)]
pub enum ConstraintTypeOpEnum {
    BoolEq,

    ListAll,
    ListSome,
    
    
    NumericEq,
    NumericNEq,

    NumericGt,
    NumericGEq,

    NumericLt,
    NumericLEq,
}