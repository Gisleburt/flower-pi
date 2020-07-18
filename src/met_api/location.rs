use serde::Deserialize;

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
