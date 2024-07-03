use klickhouse::{DateTime, Row};

#[derive(Row, Debug, Default)]
pub struct Opensky {
    callsign: String,
    number: String,
    icao24: String,
    registration: String,
    typecode: String,
    origin: String,
    destination: String,
    firstseen: DateTime,
    lastseen: DateTime,
    day: DateTime,
    latitude_1: f64,
    longitude_1: f64,
    altitude_1: f64,
    latitude_2: f64,
    longitude_2: f64,
    altitude_2: f64,
}

impl Opensky {
    pub fn schema() -> String {
        "callsign String, number String, icao24 String, registration String, typecode String, origin String, destination String, firstseen DateTime, lastseen DateTime, day DateTime, latitude_1 Float64, longitude_1 Float64, altitude_1 Float64, latitude_2 Float64, longitude_2 Float64, altitude_2 Float64"
            .to_string()
    }
}
