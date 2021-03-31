// https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x/31193386
// https://stackoverflow.com/questions/37970355/read-xml-file-into-struct
// https://serde.rs/attr-default.html, default values in struct.

#![allow(non_snake_case)]

mod structs;
mod test;

//use std::fs;

#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_xml_rs;

use dotenv::dotenv;
use reqwest::header::AUTHORIZATION;
use std::env;
use std::process::exit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*    let filename = "../stationer.xml";
    let content= fs::read_to_string(filename).expect("Unable to read file");*/

    dotenv().ok();

    let BASIC_AUTH = env::var("VEGVESEN_BASIC_AUTH").expect("Please add basic authentication");

    let client = reqwest::blocking::Client::new();

    // Check if URL is available.
    let res = client.get("http://localhost:8080").send()?;

    if !res.status().is_success() {
        exit(1);
    }

    let res = client
        .get("https://www.vegvesen.no/ws/no/vegvesen/veg/trafikkpublikasjon/vaer/2/GetMeasurementWeatherSiteTable")
        .header(AUTHORIZATION, BASIC_AUTH)
        .send()?;

    let body = res.text().unwrap();

    let mut locations: Vec<structs::WeatherStations> = Vec::new();

    let d2LogicalModel: structs::D2LogicalModel = serde_xml_rs::from_str(&body).unwrap();
    let publication_time = &d2LogicalModel
        .payloadPublication
        .publicationTime
        .publicationTime;
    println!("publication time: {}", publication_time);
    for site in &d2LogicalModel
        .payloadPublication
        .measurementSiteTable
        .measurementSiteRecord
    {
        let id = &site.id;
        let name = &site.measurementSiteName.values.value.value;
        let latitude = &site
            .measurementSiteLocation
            .pointByCoordinates
            .pointCoordinates
            .latitude
            .latitude;
        let longitude = &site
            .measurementSiteLocation
            .pointByCoordinates
            .pointCoordinates
            .longitude
            .longitude;
        let ws = structs::WeatherStations {
            publication_time: publication_time.clone(),
            id: id.clone(),
            name: name.clone(),
            latitude: *latitude,
            longitude: *longitude,
        };
        locations.push(ws);
        /*println!("publication time: {}, id: {}, name: {}, latitude: {}, longitude: {}",
        publication_time, id, name, latitude, longitude);*/
    }

    let jl = serde_json::to_string(&locations)?;
    //    println!("{}", &jl);

    let res = client
        .post("http://localhost:8080/weather_stations")
        .body(jl)
        .send()?;

    println!("res: {}", res.status());

    Ok(())
}
