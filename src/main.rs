use std::error::Error;
use isahc::prelude::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const MET_BASE: &str = "http://datapoint.metoffice.gov.uk/public/data";

struct MetApi {
    pub api_key: String,
}

impl MetApi {
    pub fn new() -> MetApi {
        MetApi {
            api_key: std::env::var("API_KEY").expect("API_KEY not found in environment"),
        }
    }

    pub fn site_list(&self) -> String {
        isahc::get(format!("{}/val/wxfcs/all/json/sitelist?key={}", MET_BASE, self.api_key))
            .expect("Could not contact met office")
            .text()
            .expect("Could not read response")
    }
}


fn main() {
    let met_api = MetApi::new();
    println!("{:#}!", met_api.site_list());
}
