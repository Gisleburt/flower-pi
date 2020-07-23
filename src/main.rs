mod clock;
mod led;
mod pollen;

use crate::led::{LedClock, LedInterface, LedValue};
use crate::pollen::{get_pollen_count, PollenCount};
use core::fmt;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::thread;
use std::time::Duration;
use crate::clock::Clock;

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

impl TryFrom<PollenCount> for LedValue {
    type Error = Box<dyn Error>;

    fn try_from(count: PollenCount) -> Result<Self> {
        Ok(match count {
            PollenCount::High => LedValue::new(1, 255, 0, 0)?, // Red
            PollenCount::Medium => LedValue::new(1, 255, 200, 0)?, // Yellow
            PollenCount::Low => LedValue::new(1, 0, 255, 0)?, // Green
        })
    }
}

fn wrapper() -> Result<()> {
    let mut interface = LedInterface::new(24)?;
    let clock = Clock::new();

    let mut led_clock = LedClock::new(24, 12, clock);
    let background = get_pollen_count()?.try_into()?;
    led_clock.set_background(background).update()?;

    loop {
        led_clock.update()?;
        interface.write(&led_clock)?.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

    // let mut led_array = LedArray::new(24);
    //
    // let mut pos = 0;
    //
    // loop {
    //     pos = (pos + 1) % 24;
    //     led_array.reset();
    //     led_array.set_led(pos, LedValue::new(1, 255, 0, 0)?)?;
    //     led_array.set_led((pos + 1) % 24, LedValue::new(1, 0, 255, 0)?)?;
    //     led_array.set_led((pos + 2) % 24, LedValue::new(1, 0, 0, 255)?)?;
    //     interface.write(&led_array)?.flush()?;
    //     thread::sleep(Duration::from_millis(50));
    // }
}
