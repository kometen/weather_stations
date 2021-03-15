// https://riptutorial.com/rust/example/4274/read-a-file-as-a-whole-as-a-string

use std::fs::File;
use std::io::Read;
use std::fmt::Arguments;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_xml_rs;

#[derive(Deserialize, Debug)]
struct D2LogicalModel {
    modelBaseVersion: u8,
    exchange: Exchange,
    payloadPublication: PayloadPublication,
}

// Exchange
#[derive(Deserialize, Debug)]
struct Exchange {
    supplierIdentification: SupplierIdentification,
}

#[derive(Deserialize, Debug)]
struct SupplierIdentification {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
struct Country {
    #[serde(rename = "$value")]
    country: String,
}

#[derive(Deserialize, Debug)]
struct NationalIdentifier {
    #[serde(rename = "$value")]
    identifier: String,
}

#[derive(Deserialize, Debug)]
struct PayloadPublication {
    lang: String,
    publicationTime: PublicationTime,
    publicationCreator: PublicationCreator,
    headerInformation: HeaderInformation,
    measurementSiteTable: MeasurementSiteTable,
}

#[derive(Deserialize, Debug)]
struct PublicationTime {
    #[serde(rename = "$value")]
    publicationTime: String,
}

#[derive(Deserialize, Debug)]
struct PublicationCreator {
    country: Country,
    nationalIdentifier: NationalIdentifier,
}

#[derive(Deserialize, Debug)]
struct HeaderInformation {
    confidentiality: Confidentiality,
    informationStatus: InformationStatus
}

#[derive(Deserialize, Debug)]
struct Confidentiality {
    #[serde(rename = "$value")]
    confidentiality: String,
}

#[derive(Deserialize, Debug)]
struct InformationStatus {
    #[serde(rename = "$value")]
    informationStatus: String,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteTable {
    id: String,
    version: u64,
    measurementSiteTableIdentification: MeasurementSiteTableIdentification,
    measurementSiteRecord: Vec<MeasurementSiteRecord>,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteTableIdentification {
    #[serde(rename = "$value")]
    measurementSiteTableIdentification: String,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteRecord {
    id: u16,
    version: u16,
    measurementSiteName: MeasurementSiteName,
    measurementSiteLocation: MeasurementSiteLocation,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteName {
    values: Values,
}

#[derive(Deserialize, Debug)]
struct Values {
    value: Value,
}

#[derive(Deserialize, Debug)]
struct Value {
    lang: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteLocation {
    pointByCoordinates: PointByCoordinates,
}

#[derive(Deserialize, Debug)]
struct PointByCoordinates {
    pointCoordinates: PointCoordinates,
}

#[derive(Deserialize, Debug)]
struct PointCoordinates {
    latitude: Latitude,
    longitude: Longitude,
}

#[derive(Deserialize, Debug)]
struct Latitude {
    #[serde(rename = "$value")]
    latitude: f32,
}

#[derive(Deserialize, Debug)]
struct Longitude {
    #[serde(rename = "$value")]
    longitude: f32,
}

struct WeatherStations {
    publication_time: String,
    id: u16,
    name: String,
    latitude: f32,
    longitude: f32,
}

impl WeatherStations {
    fn new(
        publication_time: String,
        id: u16,
        name: String,
        latitude: f32,
        longitude: f32,
    ) -> Self {
        WeatherStations {
            publication_time,
            id,
            name,
            latitude,
            longitude,
        }
    }
}
fn main() {
    let filename = "../stationer.xml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content= String::new();

            file.read_to_string(&mut content).unwrap();

            let d2LogicalModel: D2LogicalModel = serde_xml_rs::from_str(&content).unwrap();
            let publication_time = &d2LogicalModel.payloadPublication.publicationTime.publicationTime;
            println!("publication time: {}", publication_time);
            for site in &d2LogicalModel.payloadPublication.measurementSiteTable.measurementSiteRecord {
                let id = &site.id;
                let name = &site.measurementSiteName.values.value.value;
                let latitude = &site.measurementSiteLocation.pointByCoordinates.pointCoordinates.latitude.latitude;
                let longitude = &site.measurementSiteLocation.pointByCoordinates.pointCoordinates.longitude.longitude;
                println!("publication time: {}, id: {}, name: {}, latitude: {}, longitude: {}",
                         publication_time, id, name, latitude, longitude);
            }
        },
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
        }
    }
}
