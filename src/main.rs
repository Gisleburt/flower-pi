mod led;
mod pollen;

use crate::led::{LedArray, LedValue};
use crate::pollen::get_pollen_count;
use core::fmt;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct DumbError(String);

impl Error for DumbError {}

impl fmt::Display for DumbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    wrapper().unwrap();
}

fn wrapper() -> Result<()> {
    println!("{}", get_pollen_count()?);
    let mut led_array = LedArray::new(24)?;

    let mut pos = 5;

    loop {
        pos = (pos + 1) % 24;
        led_array.clear_back_buffer();
        led_array.set_led(pos, LedValue::new(1, 255, 0, 0)?)?;
        led_array.set_led((pos + 1) % 24, LedValue::new(1, 0, 255, 0)?)?;
        led_array.set_led((pos + 2) % 24, LedValue::new(1, 0, 0, 255)?)?;
        led_array.render()?;
    }
}
