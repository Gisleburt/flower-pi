mod location;

use isahc::prelude::*;

pub use location::Location;
use location::LocationsResponse;

const MET_BASE: &str = "http://datapoint.metoffice.gov.uk/public/data";

pub struct MetApi {
    pub api_key: String,
}

impl MetApi {
    pub fn new() -> MetApi {
        MetApi {
            api_key: std::env::var("API_KEY").expect("API_KEY not found in environment"),
        }
    }

    pub fn site_list(&self) -> Vec<Location> {
        let response: LocationsResponse = isahc::get(format!(
            "{}/val/wxfcs/all/json/sitelist?key={}",
            MET_BASE, self.api_key
        ))
        .expect("Could not contact met office")
        .json()
        .expect("Could not read response");
        response.locations.location
    }
}
