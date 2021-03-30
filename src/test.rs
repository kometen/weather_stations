#[cfg(test)]
mod tests {
    use crate::structs;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_xml() {
        let body = r#"<d2LogicalModel modelBaseVersion="3" xmlns="http://datex2.eu/schema/2/2_0">
    <exchange>
        <supplierIdentification>
            <country>no</country>
            <nationalIdentifier>Norwegian Public Roads Administration</nationalIdentifier>
        </supplierIdentification>
    </exchange>
    <payloadPublication lang="nob" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="MeasurementSiteTablePublication">
        <publicationTime>2021-03-05T11:22:01.628+01:00</publicationTime>
        <publicationCreator>
            <country>no</country>
            <nationalIdentifier>Norwegian Public Roads Administration</nationalIdentifier>
        </publicationCreator>
        <headerInformation>
            <confidentiality>noRestriction</confidentiality>
            <informationStatus>real</informationStatus>
        </headerInformation>
        <measurementSiteTable id="WOST" version="20210305111835000">
            <measurementSiteTableIdentification>WEATHER_OBSERVATION_STATION_TABLE</measurementSiteTableIdentification>
            <measurementSiteRecord id="205" version="70">
                <measurementSiteName>
                    <values>
                        <value lang="nob">E6 Rosten</value>
                    </values>
                </measurementSiteName>
                <measurementSpecificCharacteristics index="101">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>temperatureInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="201">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>humidityInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="301">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>temperatureInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="801">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>roadSurfaceConditionInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="701">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>precipitationInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="1401">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>precipitationInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="2201">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>precipitationInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSpecificCharacteristics index="2501">
                    <measurementSpecificCharacteristics>
                        <specificMeasurementValueType>precipitationInformation</specificMeasurementValueType>
                    </measurementSpecificCharacteristics>
                </measurementSpecificCharacteristics>
                <measurementSiteLocation xsi:type="Point">
                    <pointByCoordinates>
                        <pointCoordinates>
                            <latitude>61.878395</latitude>
                            <longitude>9.41545</longitude>
                        </pointCoordinates>
                    </pointByCoordinates>
                </measurementSiteLocation>
            </measurementSiteRecord>
        </measurementSiteTable>
    </payloadPublication>
</d2LogicalModel>""#;

        let json = r#"[{"publication_time":"2021-03-05T11:22:01.628+01:00","id":205,"name":"E6 Rosten","latitude":61.878395,"longitude":9.41545}]"#.to_string();

        let mut locations: Vec<structs::WeatherStations> = Vec::new();

        let d2LogicalModel: structs::D2LogicalModel = serde_xml_rs::from_str(&body).unwrap();
        let publication_time = &d2LogicalModel.payloadPublication.publicationTime.publicationTime;
        println!("publication time: {}", publication_time);
        for site in &d2LogicalModel.payloadPublication.measurementSiteTable.measurementSiteRecord {
            let id = &site.id;
            let name = &site.measurementSiteName.values.value.value;
            let latitude = &site.measurementSiteLocation.pointByCoordinates.pointCoordinates.latitude.latitude;
            let longitude = &site.measurementSiteLocation.pointByCoordinates.pointCoordinates.longitude.longitude;
            let ws = structs::WeatherStations {
                publication_time: publication_time.clone(),
                id: id.clone(),
                name: name.clone(),
                latitude: latitude.clone(),
                longitude: longitude.clone(),
            };
            locations.push(ws);
            assert_eq!(publication_time, "2021-03-05T11:22:01.628+01:00");
            assert_eq!(id.clone(), 205);
            assert_eq!(name, "E6 Rosten");
            assert_eq!(latitude.clone(), 61.878395);
            assert_eq!(longitude.clone(), 9.41545);
            /*println!("publication time: {}, id: {}, name: {}, latitude: {}, longitude: {}",
                     publication_time, id, name, latitude, longitude);*/

            let optional = Some(serde_json::to_string(&locations));
            match optional {
                Some(jl) => {
                    assert_eq!(jl.unwrap(), json);
                },
                _ => {println!("test")},
            }
        }
    }
}
