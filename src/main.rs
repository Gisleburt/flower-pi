mod met_api;

use std::error::Error;

use met_api::MetApi;
use crate::met_api::LocationId;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let met_api = MetApi::new();
    let location_id = LocationId::Location(352677);
    let forecast = met_api.forecast(location_id);

    println!("{:#?}", forecast);

    let locations = met_api.forecast_site_list();
    locations
        .into_iter()
        .filter(|location| location.name.contains("Morden"))
        .for_each(|location| println!("{:#?}", location))
}
