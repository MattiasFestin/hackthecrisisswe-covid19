use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
// use std::thread;
use r2d2_redis::redis::Commands;


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Matchquality {
    #[serde(rename = "matchcode", skip_serializing_if = "Option::is_none")]
    pub matchcode: Option<String>,
    #[serde(rename = "matchtype", skip_serializing_if = "Option::is_none")]
    pub matchtype: Option<String>,
    #[serde(rename = "matchlevel", skip_serializing_if = "Option::is_none")]
    pub matchlevel: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "house_number", skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,
    #[serde(rename = "road", skip_serializing_if = "Option::is_none")]
    pub road: Option<String>,
    #[serde(rename = "residential", skip_serializing_if = "Option::is_none")]
    pub residential: Option<String>,
    #[serde(rename = "borough", skip_serializing_if = "Option::is_none")]
    pub borough: Option<String>,
    #[serde(rename = "neighbourhood", skip_serializing_if = "Option::is_none")]
    pub neighbourhood: Option<String>,
    #[serde(rename = "quarter", skip_serializing_if = "Option::is_none")]
    pub quarter: Option<String>,
    #[serde(rename = "hamlet", skip_serializing_if = "Option::is_none")]
    pub hamlet: Option<String>,
    #[serde(rename = "suburb", skip_serializing_if = "Option::is_none")]
    pub suburb: Option<String>,
    #[serde(rename = "island", skip_serializing_if = "Option::is_none")]
    pub island: Option<String>,
    #[serde(rename = "village", skip_serializing_if = "Option::is_none")]
    pub village: Option<String>,
    #[serde(rename = "town", skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "city_district", skip_serializing_if = "Option::is_none")]
    pub city_district: Option<String>,
    #[serde(rename = "county", skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "state_district", skip_serializing_if = "Option::is_none")]
    pub state_district: Option<String>,
    #[serde(rename = "postcode", skip_serializing_if = "Option::is_none")]
    pub postcode: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "country_code", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "state_code", skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Location {
    #[serde(rename = "distance", skip_serializing_if = "Option::is_none")]
    pub distance: Option<f32>,
    #[serde(rename = "place_id", skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    #[serde(rename = "licence", skip_serializing_if = "Option::is_none")]
    pub licence: Option<String>,
    #[serde(rename = "osm_type", skip_serializing_if = "Option::is_none")]
    pub osm_type: Option<String>,
    #[serde(rename = "osm_id", skip_serializing_if = "Option::is_none")]
    pub osm_id: Option<String>,
    #[serde(rename = "boundingbox", skip_serializing_if = "Option::is_none")]
    pub boundingbox: Option<Vec<String>>,
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    #[serde(rename = "lon", skip_serializing_if = "Option::is_none")]
    pub lon: Option<String>,
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "class", skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "importance", skip_serializing_if = "Option::is_none")]
    pub importance: Option<f32>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "namedetails", skip_serializing_if = "Option::is_none")]
    pub namedetails: Option<HashMap<String, String>>,
    #[serde(rename = "matchquality", skip_serializing_if = "Option::is_none")]
    pub matchquality: Option<Matchquality>,
}

pub fn getAddress(lat: f64, lng: f64, redis: &mut r2d2_redis::redis::Connection) -> Result<Option<Address>, Box<dyn std::error::Error>> {
     dotenv().ok();

	let locationiq_key = env::var("LocationIQKey")
        .expect("LocationIQKey must be set");

    // let mut redis = crate::db::establish_redis_connection().get().unwrap();
        
    let key = format!("{},{}", lat, lng);

    let cached: redis::RedisResult<String> = redis.get(&key);
    match cached {
        Ok(data) => {
            let deserialized: Address = serde_json::from_str(&data)?;
            return Ok(Some(deserialized));
        },
        _ => ()
    }

    // let redis: &'r r2d2_redis::redis::Connection = redis;
    // thread::spawn(move || {
        let url = format!("https://eu1.locationiq.com/v1/reverse.php?key={}&lat={}&lon={}&format=json",
            locationiq_key,
            lat,
            lng
        );

        let resp = reqwest::blocking::get(&url).unwrap()
            .json::<Location>().unwrap();

        if let Some(address) = resp.address {
            let serialized = serde_json::to_string(&address).unwrap();
            redis.set::<&String, &String, ()>(&key, &serialized).unwrap();
        }
    // });

	return Ok(None);
}

// fn getCache<'a, T: serde::Deserialize<'a>>(redis: &redis::Connection, key: String) -> Option<T> {
//     // let mut con = apiio.redis.get_connection()?;

//     // let data: String = apiio.redis.get<String>(key)?;
//     // let deserialized: T = serde_json::from_str(&data);
//     let data: redis::RedisResult<String> = redis.get(&key);
//     match cached {
//         Ok(data) => {
//             let result: Result<T, serde_json::error::Error> = serde_json::from_str(data);
//             match result {
//                 Ok(deserialized) => {
//                     println!("Cache hit");
//                     return Some(deserialized);
//                 },
//                 _ => {
//                     return None;
//                 }
//             }
//         },
//         _ => {
//             return None;
//         }
//     }

// //     return Box::new(deserialized);
// }

// fn setCache<T: serde::Serialize>(apiio: crate::db::ApiIO, data: T, key: String) -> Box<Result<dyn Option,dyn std::error::Error>> {
//     let serialized = serde_json::to_string(&data).unwrap();
    
//     // let mut con = apiio.redis.get_connection()?;
//     let _ : () = apiio.redis.set(key, serialized)?;

//     return Box::new(None);
// }