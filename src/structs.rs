// XML-root
#[derive(Deserialize, Debug)]
pub(crate) struct D2LogicalModel {
    modelBaseVersion: u8,
    exchange: Exchange,
    pub(crate) payloadPublication: PayloadPublication,
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
pub(crate) struct PayloadPublication {
    lang: String,
    pub(crate) publicationTime: PublicationTime,
    publicationCreator: PublicationCreator,
    headerInformation: HeaderInformation,
    pub(crate) measurementSiteTable: MeasurementSiteTable,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PublicationTime {
    #[serde(rename = "$value")]
    pub(crate) publicationTime: String,
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
pub(crate) struct MeasurementSiteTable {
    id: String,
    version: u64,
    measurementSiteTableIdentification: MeasurementSiteTableIdentification,
    pub(crate) measurementSiteRecord: Vec<MeasurementSiteRecord>,
}

#[derive(Deserialize, Debug)]
struct MeasurementSiteTableIdentification {
    #[serde(rename = "$value")]
    measurementSiteTableIdentification: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteRecord {
    pub(crate) id: u16,
    version: u16,
    pub(crate) measurementSiteName: MeasurementSiteName,
    pub(crate) measurementSiteLocation: MeasurementSiteLocation,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteName {
    pub(crate) values: Values,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Values {
    pub(crate) value: Value,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Value {
    lang: String,
    #[serde(rename = "$value")]
    pub(crate) value: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct MeasurementSiteLocation {
    pub(crate) pointByCoordinates: PointByCoordinates,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PointByCoordinates {
    pub(crate) pointCoordinates: PointCoordinates,
}

#[derive(Deserialize, Debug)]
pub(crate) struct PointCoordinates {
    pub(crate) latitude: Latitude,
    pub(crate) longitude: Longitude,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Latitude {
    #[serde(rename = "$value")]
    pub(crate) latitude: f32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Longitude {
    #[serde(rename = "$value")]
    pub(crate) longitude: f32,
}

#[derive(Serialize)]
pub(crate) struct WeatherStations {
    pub(crate) publication_time: String,
    pub(crate) id: u16,
    pub(crate) name: String,
    pub(crate) latitude: f32,
    pub(crate) longitude: f32,
}
