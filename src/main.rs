use core::fmt;
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use isahc::ResponseExt;
use scraper::{Html, Selector};
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{
    convert::{TryFrom, TryInto},
    error::Error as StdError,
    io::Write,
    thread,
    time::Duration,
};

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

struct LedValue {
    brightness: u8,
    blue: u8,
    green: u8,
    red: u8,
}

type LedMessage =  [u8;4];

impl LedValue {
    pub fn new(brightness: u8, red: u8, green: u8, blue: u8) -> Result<LedValue> {
        if brightness > 31 {
            Err(DumbError("brightness can not be higher than 31".to_string()).into())
        } else {
            Ok(LedValue { brightness, red, green, blue })
        }
    }

    pub fn as_array(&self) -> LedMessage {
        [self.brightness + 224, self.blue, self.green, self.red]
    }
}

struct LedArray {
    size: u8,
    spi: Spidev,
}

impl LedArray {
    fn new(size: u8) -> Result<LedArray> {
        let mut spi = Spidev::open("/dev/spidev0.0")?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(30_000_000)
            .mode(SpiModeFlags::SPI_MODE_1)
            .build();
        spi.configure(&options)?;
        let mut led_array = LedArray { size, spi };
        led_array.flush()?;
        Ok(led_array)
    }

    fn render(&mut self, led_values: &Vec<LedValue>) -> Result<()> {
        for led_value in led_values {
            let values = led_value.as_array();
            self.spi.write(&values)?;
        }
        self.flush()?;
        thread::sleep(Duration::from_millis(10_000));
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        let null_message: LedMessage = [0, 0, 0, 0];
        self.spi.write(&null_message)?;
        Ok(())
    }
}

fn main() {
    // println!("{}", get_pollen_count().unwrap());
    let mut led_array = LedArray::new(24).unwrap();
    println!("Lights on");
    led_array.render(&vec![
        LedValue::new(6, 255, 255, 255, ).unwrap(),
        LedValue::new(6, 255, 0, 0, ).unwrap(),
        LedValue::new(6, 0, 255, 0, ).unwrap(),
        LedValue::new(6, 0, 0, 255, ).unwrap(),
        LedValue::new(31, 255, 255, 255, ).unwrap(),
    ]).unwrap();
    println!("Lights off");
}
