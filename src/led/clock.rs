use crate::clock::Clock;
use crate::error::{FlowerError, Result};
use crate::led::interface::LedWritable;
use crate::led::LedValue;
use crate::led::value::{LED_LOW_PURPLE, LED_LOW_BLUE, LED_LOW_AQUA};

pub struct LedClock {
    clock: Clock,
    background: LedValue,
    hour: LedValue,
    minute: LedValue,
    second: LedValue,
    led_buffer: Vec<LedValue>,
    led_offset: usize,
}

impl LedClock {
    pub fn new(num_leds: usize, led_offset: usize, clock: Clock) -> LedClock {
        LedClock {
            clock,
            led_offset,
            background: LedValue::default(),
            hour: LED_LOW_PURPLE.clone(),
            minute: LED_LOW_BLUE.clone(),
            second: LED_LOW_AQUA.clone(),
            led_buffer: vec![LedValue::default(); num_leds],
        }
    }

    pub fn set_background(&mut self, background: LedValue) -> &mut Self {
        self.background = background;
        self
    }

    fn set_led(&mut self, led_num: usize, value: LedValue) -> Result<&mut Self> {
        self.led_buffer
            .get_mut(led_num)
            .map(|led| *led = value)
            .ok_or_else(|| {
                FlowerError::SimpleError(format!(
                    "Invalid index {} in LedArray of size {}",
                    led_num,
                    self.led_buffer.len()
                ))
            })?;
        Ok(self)
    }

    fn fit_index_to_buffer(&self, index: usize, divisor: usize) -> usize {
        (((index * self.led_buffer.len()) / divisor) + self.led_offset) % self.led_buffer.len()
    }

    pub fn update(&mut self) -> Result<&mut Self> {
        self.led_buffer = vec![self.background.clone(); self.led_buffer.len()];
        let hours = self.fit_index_to_buffer(self.clock.get_hours(), 12);
        let minutes = self.fit_index_to_buffer(self.clock.get_minutes(), 60);
        let seconds = self.fit_index_to_buffer(self.clock.get_seconds(), 60);
        self.set_led(hours, self.hour)?;
        self.set_led(hours + 1, self.hour)?;
        self.set_led(minutes, self.minute)?;
        self.set_led(seconds, self.second)?;
        Ok(self)
    }
}

impl LedWritable for LedClock {
    fn as_array(&self) -> &[LedValue] {
        self.led_buffer.as_slice()
    }
}
