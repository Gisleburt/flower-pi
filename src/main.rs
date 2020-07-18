mod met_api;

use std::error::Error;

use met_api::MetApi;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() {
    let met_api = MetApi::new();
    let locations = met_api.site_list();
    locations
        .into_iter()
        .filter(|location| location.name.contains("Morden"))
        .for_each(|location| println!("{:#?}", location))
}
