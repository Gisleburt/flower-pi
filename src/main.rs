use core::fmt;
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use isahc::ResponseExt;
use scraper::{Html, Selector};
use std::{
    convert::{TryFrom, TryInto},
    error::Error as StdError,
    io::Write,
    thread,
    time::Duration,
};
use rppal::spi::{Spi, Bus, SlaveSelect, Mode};

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

#[derive(Clone, Debug)]
struct LedValue {
    brightness: u8,
    blue: u8,
    green: u8,
    red: u8,
}

type LedMessage = [u8; 4];

impl LedValue {
    pub fn new(brightness: u8, red: u8, green: u8, blue: u8) -> Result<LedValue> {
        if brightness > 31u8 {
            Err(DumbError(
                "brightness can not be higher than 31".to_string(),
            ))?;
        }
        Ok(LedValue {
            brightness,
            red,
            green,
            blue,
        })
    }

    pub fn as_array(&self) -> LedMessage {
        const BRIGHTNESS_MOD: u8 = 224;
        let brightness = self.brightness + BRIGHTNESS_MOD;
        [brightness, self.blue, self.green, self.red]
    }
}

impl Default for LedValue {
    fn default() -> Self {
        LedValue {
            brightness: 0,
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

struct LedArray {
    back_buffer: Vec<LedValue>,
    spi: Spi,
}

impl LedArray {
    fn new(size: u8) -> Result<LedArray> {
        let mut spi = Spi::new(
            Bus::Spi0,
            SlaveSelect::Ss1,
            30_000_000,
            Mode::Mode0,
        )?;
        let back_buffer = vec![LedValue::default(); size as usize];
        let mut led_array = LedArray { back_buffer, spi };
        led_array.flush()?;
        Ok(led_array)
    }

    fn render(&mut self) -> Result<()> {
        for led_value in &self.back_buffer {
            let values = led_value.as_array();
            self.spi.write(&values)?;
        }
        self.flush()?;
        thread::sleep(Duration::from_millis(10_000));
        Ok(())
    }

    fn clear_back_buffer(&mut self) {
        self.back_buffer = vec![LedValue::default(); self.back_buffer.len()];
    }

    fn set_led(&mut self, num: u8, value: LedValue) -> Result<()> {
        if num as usize >= self.back_buffer.len() {
            Err(DumbError(
                "brightness can not be higher than 31".to_string(),
            ))?;
        }
        self.back_buffer[num as usize] = value;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        const NULL_MESSAGE: LedMessage = [0, 0, 0, 0];
        let num_flushes = (self.back_buffer.len() / 2) + 2;
        for _ in 0..num_flushes {
            self.spi.write(&NULL_MESSAGE)?;
        }
        Ok(())
    }
}

fn main() {
    wrapper().unwrap();
}

fn wrapper() -> Result<()> {
    // println!("{}", get_pollen_count()?);
    let mut led_array = LedArray::new(24)?;

    led_array.set_led(1, LedValue::new(1, 255, 255, 255)?)?;
    led_array.set_led(2, LedValue::new(1, 255, 0, 0)?)?;
    led_array.set_led(3, LedValue::new(1, 0, 255, 0)?)?;
    led_array.set_led(4, LedValue::new(1, 0, 0, 255)?)?;
    led_array.set_led(5, LedValue::new(1, 255, 255, 255)?)?;
    println!("Lights on");
    led_array.render()?;
    led_array.clear_back_buffer();
    println!("Lights off");
    led_array.render()?;
    Ok(())
}
