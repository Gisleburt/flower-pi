use crate::error::FlowerError;
use crate::Result;
use core::{
    convert::{TryFrom, TryInto},
    fmt,
};
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use scraper::{Html, Selector};
use std::error::Error as StdError;

#[derive(Debug)]
pub struct PollenParseError(String);

impl StdError for PollenParseError {}

impl fmt::Display for PollenParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse pollen count: {}", self.0)
    }
}

#[derive(Debug)]
pub enum PollenCount {
    High,
    Medium,
    Low,
}

impl fmt::Display for PollenCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::High => write!(f, "High"),
            Self::Medium => write!(f, "Medium"),
            Self::Low => write!(f, "Low"),
        }
    }
}

impl TryFrom<&str> for PollenCount {
    type Error = PollenParseError;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "h" => Ok(PollenCount::High),
            "m" => Ok(PollenCount::Medium),
            "l" => Ok(PollenCount::Low),
            x => Err(PollenParseError(x.to_string())),
        }
    }
}

const POLLEN_URL: &str =
    "https://metoffice.gov.uk/weather/warnings-and-advice/seasonal-advice/pollen-forecast";

fn get_html() -> Result<String> {
    Ok(Request::get(POLLEN_URL)
        .redirect_policy(RedirectPolicy::Follow)
        .body(())?
        .send()?
        .text()?)
}

pub fn get_pollen_count() -> Result<PollenCount> {
    let html = get_html()?;

    let document = Html::parse_document(html.as_str());
    //*[@id="se"]/table/tbody/tr/td[1]/div/span
    let se_selector = Selector::parse("#se")
        .map_err(|_| FlowerError::SimpleError("Could not create #se parser".to_string()))?;
    let se = document
        .select(&se_selector)
        .next()
        .ok_or_else(|| FlowerError::SimpleError("#se not found on page".to_string()))?;

    let span_selector = Selector::parse("span")
        .map_err(|_| FlowerError::SimpleError("Could not create #se parser".to_string()))?;
    let today = se.select(&span_selector).next().ok_or_else(|| {
        FlowerError::SimpleError("today span (the first span) was not found".to_string())
    })?;
    let pollen_indicator = today.value().attr("data-category").ok_or_else(|| {
        FlowerError::SimpleError("No data-category attribute found on today span".to_string())
    })?;
    Ok(pollen_indicator.try_into()?)
}
