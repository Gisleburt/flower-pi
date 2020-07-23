use crate::led::{LedMessage, LedValue, LedWritable};
use crate::{DumbError, Result};

const NULL_MESSAGE: LedMessage = [0, 0, 0, 0];

pub struct LedArray {
    background: LedValue,
    led_buffer: Vec<LedValue>,
}

impl LedArray {
    pub fn new(size: usize) -> LedArray {
        LedArray {
            background: LedValue::default(),
            led_buffer: vec![LedValue::default(); size],
        }
    }

    pub fn set_background(&mut self, background: LedValue) -> &mut Self {
        self.background = background;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.led_buffer = vec![self.background.clone(); self.led_buffer.len()];
        self
    }

    pub fn set_led(&mut self, led_num: usize, value: LedValue) -> Result<&mut Self> {
        self.led_buffer.get_mut(led_num).map(|led| *led = value).ok_or_else(|| {
            DumbError(format!("Invalid index {} in LedArray of size {}", led_num, self.led_buffer.len()))
        })?;
        Ok(self)
    }
}

impl LedWritable for LedArray {
    fn as_array(&self) -> &[LedValue] {
        self.led_buffer.as_slice()
    }
}
