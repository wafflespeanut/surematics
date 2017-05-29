extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;

use std::env;
use std::f64::consts::PI;
use std::fmt::Display;

// This could also be changed to a local database consisting of geocodes (should it?).
const API_ENDPOINT: &'static str = "https://api.postcodes.io/postcodes/";
const EARTH_RADIUS: f64 = 6_378_137.0;      // in meters

#[derive(Debug)]
struct UKLocation {
    lat: f64,
    long: f64,
}

impl UKLocation {
    fn from_code<S>(code: S) -> Result<Self, String> where S: Display {
        let client = Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        let url = format!("{}{}", API_ENDPOINT, code);
        let mut response = client.get(&url)
                                 .send()
                                 .map_err(|e| format!("Cannot connect to server! ({})", e))?;
        let value: Value = serde_json::from_reader(&mut response)
                                      .map_err(|e| format!("Cannot decode JSON data! ({})", e))?;
        if value["status"].as_u64().unwrap() != 200 {
            return Err(format!("{}: {}", code, value["error"].as_str().unwrap()))
        }

        let ref result = value["result"];
        Ok(UKLocation {
            lat: result["latitude"].as_f64().unwrap() * PI / 180.0,
            long: result["longitude"].as_f64().unwrap() * PI / 180.0,
        })
    }

    // Haversine formula (https://en.wikipedia.org/wiki/Haversine_formula)
    // Note that the WGS format (returned by the API) for latitude/longitude is based on
    // an ellipsoid whereas this assumes a sphere (the resulting error isn't much).
    fn dist_from(&self, other: &UKLocation) -> f64 {
        let dlat = self.lat - other.lat;
        let dlong = self.long - other.long;
        let hav_lat = (dlat / 2.0).sin().powi(2);
        let hav_long = (dlong / 2.0).sin().powi(2);
        let angle = hav_lat + self.lat.cos() * other.lat.cos() * hav_long;
        (2.0 * angle.sqrt().atan2((1.0 - angle).sqrt()) * EARTH_RADIUS) / 1000.0
    }
}

fn parse_args_and_find() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let (loc_1, loc_2) = match (args.next(), args.next()) {
        (Some(c_1), Some(c_2)) => (UKLocation::from_code(c_1)?, UKLocation::from_code(c_2)?),
        _ => return Err(String::from("Please pass two postal codes as arguments"))
    };

    println!("Distance: {} km", loc_1.dist_from(&loc_2));
    Ok(())
}

fn main() {
    if let Err(s) = parse_args_and_find() {
        println!("{}", s);
    }
}
