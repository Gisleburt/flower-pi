mod forecast;
mod location;
mod sane;

use isahc::prelude::*;

pub use location::{Location, LocationId};
use location::LocationsResponse;
use crate::met_api::forecast::ForecastResponse;

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

    fn make_request(&self, uri: String) -> Response<Body> {
        isahc::get(uri)
            .expect("Could not contact met office")
    }

    pub fn forecast_site_list(&self) -> Vec<Location> {
        let response: LocationsResponse = self.make_request(format!(
            "{}/val/wxfcs/all/json/sitelist?key={}",
            MET_BASE, self.api_key
        ))
        .json()
        .expect("Could not read response");
        response.locations.location
    }

    pub fn forecast_capabilities(&self) -> String {
        let uri = format!(
            "{}/val/wxfcs/all/json/capabilities?key={}",
            MET_BASE, self.api_key
        );
        println!("{}", uri);
        self.make_request(uri)
            .text()
            .expect("Could not read response")
    }

    pub fn forecast(&self, location_id: LocationId) -> ForecastResponse {
        let text = self.make_request(format!(
            "{}/val/wxfcs/all/json/{}?res=3hourly&key={}",
            MET_BASE, location_id, self.api_key
        ))
            .text()
            .expect("couldn't read");
        println!("{:#?}", text);
        let forecast: ForecastResponse = self.make_request(format!(
            "{}/val/wxfcs/all/json/{}?res=3hourly&key={}",
            MET_BASE, location_id, self.api_key
        ))
        .json()
        .expect("Could not read response");
        forecast
    }
}
