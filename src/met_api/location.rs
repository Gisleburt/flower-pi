use serde::export::Formatter;
use serde::Deserialize;
use std::fmt;

pub enum LocationId {
    All,
    Location(u32),
}

impl fmt::Display for LocationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            LocationId::Location(loc) => write!(f, "{}", loc),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    pub id: String,
    pub name: String,
    pub latitude: String,
    pub longitude: String,
    pub elevation: Option<String>,
    pub region: Option<String>,
    pub unitary_auth_area: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocationsResponse {
    pub locations: Locations,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Locations {
    pub location: Vec<Location>,
}
