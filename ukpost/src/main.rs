extern crate hyper;
extern crate hyper_native_tls;
extern crate serde_json;

use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::Value;

use std::env;
use std::fmt::Display;

const API_ENDPOINT: &'static str = "https://api.postcodes.io/postcodes/";

#[derive(Debug)]
struct UKLocation {
    lat: f64,
    long: f64,
}

impl UKLocation {
    fn from_code<S>(code: S) -> Result<Self, String> where S: Display {
        let client = Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        let url = format!("{}{}", API_ENDPOINT, code);
        println!("GET: {}", url);
        let mut response = client.get(&url)
                                 .send()
                                 .map_err(|e| format!("Cannot connect to server! ({})", e))?;
        let value: Value = serde_json::from_reader(&mut response)
                                      .map_err(|e| format!("Cannot decode JSON data! ({})", e))?;
        if value["status"].as_u64().unwrap() != 200 {
            return Err(value["error"].as_str().unwrap().to_owned())
        }

        let ref result = value["result"];
        Ok(UKLocation {
            lat: result["latitude"].as_f64().unwrap(),
            long: result["longitude"].as_f64().unwrap(),
        })
    }
}

fn parse_args_and_find() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let (loc_1, loc_2) = match (args.next(), args.next()) {
        (Some(c_1), Some(c_2)) => (UKLocation::from_code(c_1)?, UKLocation::from_code(c_2)),
        _ => return Err(String::from("Please pass two postal codes as arguments"))
    };

    println!("{:?}", (loc_1, loc_2));
    Ok(())
}

fn main() {
    if let Err(s) = parse_args_and_find() {
        println!("{}", s);
    }
}
