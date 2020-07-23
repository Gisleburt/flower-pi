use crate::led::interface::LedWritable;
use crate::led::LedValue;
use crate::clock::{Clock, FakeClock};
use crate::{DumbError, Result};

pub struct LedClock {
    clock: FakeClock,
    background: LedValue,
    led_buffer: Vec<LedValue>,
}

impl LedClock {
    pub fn new(num_leds: usize, clock: FakeClock) -> LedClock {
        LedClock {
            clock,
            background: LedValue::default(),
            led_buffer: vec![LedValue::default(); num_leds],
        }
    }

    pub fn set_background(&mut self, background: LedValue) -> &mut Self {
        self.background = background;
        self
    }

    fn set_led(&mut self, led_num: usize, value: LedValue) -> Result<&mut Self> {
        self.led_buffer.get_mut(led_num).map(|led| *led = value).ok_or_else(|| {
            DumbError(format!("Invalid index {} in LedArray of size {}", led_num, self.led_buffer.len()))
        })?;
        Ok(self)
    }

    pub fn update(&mut self) -> Result<&mut Self> {
        self.led_buffer = vec![self.background.clone(); self.led_buffer.len()];
        let seconds = ((self.clock.get_seconds() * self.led_buffer.len()) / 60) % 24;
        let minutes = ((self.clock.get_minutes() * self.led_buffer.len()) / 60) % 24;
        let hours = ((self.clock.get_hours() * self.led_buffer.len()) / 12) % 24;
        self.set_led(seconds, LedValue::new(1, 0, 0, 255)?)?;
        self.set_led(minutes, LedValue::new(1, 0, 0, 255)?)?;
        self.set_led(hours, LedValue::new(1, 0, 0, 255)?)?;
        self.set_led(hours + 1, LedValue::new(1, 0, 0, 255)?)?;

        self.clock.advance_one_second();// ToDo: Remove this

        Ok(self)
    }
}

impl LedWritable for LedClock {
    fn as_array(&self) -> &[LedValue] {
        self.led_buffer.as_slice()
    }
}
