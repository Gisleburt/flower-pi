use core::fmt;
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use isahc::ResponseExt;
use rppal::gpio::{Gpio, OutputPin};
use scraper::{Html, Selector};
use std::convert::{TryFrom, TryInto};
use std::error::Error as StdError;
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[derive(Debug)]
struct DumbError(String);

impl StdError for DumbError {}

impl fmt::Display for DumbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct PollenParseError(String);

impl StdError for PollenParseError {}

impl fmt::Display for PollenParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse pollen count: {}", self.0)
    }
}

#[derive(Debug)]
enum PollenCount {
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

fn get_pollen_count() -> Result<PollenCount> {
    let html = get_html()?;

    let document = Html::parse_document(html.as_str());
    //*[@id="se"]/table/tbody/tr/td[1]/div/span
    let se_selector =
        Selector::parse("#se").map_err(|_| DumbError("Could not create #se parser".to_string()))?;
    let se = document
        .select(&se_selector)
        .next()
        .ok_or_else(|| DumbError("#se not found on page".to_string()))?;

    let span_selector = Selector::parse("span")
        .map_err(|_| DumbError("Could not create #se parser".to_string()))?;
    let today = se
        .select(&span_selector)
        .next()
        .ok_or_else(|| DumbError("today span (the first span) was not found".to_string()))?;
    let pollen_indicator = today
        .value()
        .attr("data-category")
        .ok_or_else(|| DumbError("No data-category attribute found on today span".to_string()))?;
    Ok(pollen_indicator.try_into()?)
}

const GPIO17: u8 = 17;

struct LedBar {
    pins: Vec<OutputPin>,
}

impl LedBar {
    fn new(pin_numbers: &[u8]) -> LedBar {
        LedBar {
            pins: pin_numbers
                .iter()
                .map(|pin| Gpio::new().unwrap().get(*pin).unwrap().into_output())
                .collect(),
        }
    }

    fn test(&mut self) {
        self.pins.iter_mut().for_each(|pin| {
            pin.set_high();
            thread::sleep(Duration::from_millis(100));
            pin.set_low();
        });
        self.pins.iter_mut().rev().for_each(|pin| {
            pin.set_high();
            thread::sleep(Duration::from_millis(100));
            pin.set_low();
        });
    }

    fn clear(&mut self) {
        self.pins.iter_mut().for_each(|pin| {
            pin.set_low();
        });
    }
}

fn main() {
    // println!("{}", get_pollen_count().unwrap());
    let mut bar = LedBar::new(&[17, 18, 27, 22, 23, 24, 25, 12, 13, 19]);
    bar.clear();
    loop {
        bar.test();
    }
}
