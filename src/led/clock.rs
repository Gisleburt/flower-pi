use crate::led::interface::LedWritable;
use crate::led::LedValue;

pub struct LedClock {
    background: LedValue,
    led_buffer: Vec<LedValue>,
}

impl LedClock {
    pub fn new(num_leds: usize) -> LedClock {
        LedClock {
            background: LedValue::default(),
            led_buffer: vec![LedValue::default(); num_leds],
        }
    }

    pub fn set_background(&mut self, background: LedValue) -> &mut Self {
        self.background = background;
        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.led_buffer = vec![self.background.clone(); self.led_buffer.len()];
        self
    }
}

impl LedWritable for LedClock {
    fn as_array(&self) -> &[LedValue] {
        self.led_buffer.as_slice()
    }
}
