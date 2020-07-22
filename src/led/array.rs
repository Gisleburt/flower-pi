use crate::led::{LedMessage, LedValue};
use crate::{DumbError, Result};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::thread;
use std::time::Duration;

const NULL_MESSAGE: LedMessage = [0, 0, 0, 0];

pub struct LedArray {
    back_buffer: Vec<LedValue>,
    spi: Spi,
}

impl LedArray {
    pub fn new(size: u8) -> Result<LedArray> {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 30_000_000, Mode::Mode0)?;
        let back_buffer = vec![LedValue::default(); size as usize];
        let mut led_array = LedArray { back_buffer, spi };
        led_array.flush()?;
        Ok(led_array)
    }

    pub fn render(&mut self) -> Result<()> {
        for led_value in &self.back_buffer {
            let values = led_value.as_array();
            self.spi.write(&values)?;
        }
        self.flush()?;
        thread::sleep(Duration::from_millis(50));
        Ok(())
    }

    pub fn clear_back_buffer(&mut self) {
        self.back_buffer = vec![LedValue::default(); self.back_buffer.len()];
    }

    pub fn set_led(&mut self, num: u8, value: LedValue) -> Result<()> {
        if num as usize >= self.back_buffer.len() {
            Err(DumbError(
                "brightness can not be higher than 31".to_string(),
            ))?;
        }
        self.back_buffer[num as usize] = value;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.spi.write(&NULL_MESSAGE)?;
        Ok(())
    }
}
