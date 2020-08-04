use crate::error::{FlowerError, Result};
use crate::led::{LedMessage, LedValue};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const NULL_MESSAGE: LedMessage = [0, 0, 0, 0];

pub struct LedInterface {
    size: usize,
    back_buffer: Vec<LedValue>,
    spi: Spi,
}

pub trait LedWritable {
    fn as_array(&self) -> &[LedValue];
}

impl LedInterface {
    pub fn new(size: usize) -> Result<LedInterface> {
        let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 30_000_000, Mode::Mode0)?;
        // self.spi.write(&NULL_MESSAGE)?;
        let back_buffer = vec![];
        let mut led_array = LedInterface {
            back_buffer,
            spi,
            size,
        };
        led_array.flush()?;
        Ok(led_array)
    }

    pub fn write<W: LedWritable>(&mut self, writable: &W) -> Result<&mut Self> {
        let slice = writable.as_array();
        if self.back_buffer.len() + slice.len() <= self.size {
            self.back_buffer.extend_from_slice(slice);
            Ok(self)
        } else {
            Err(FlowerError::SimpleError(
                "writing this slice would overflow the back buffer".to_string(),
            )
            .into())
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.back_buffer = vec![LedValue::default(); self.size];
        self
    }

    pub fn flush(&mut self) -> Result<&mut Self> {
        // Drain the back buffer into the spi interface
        for led_value in self.back_buffer.drain(..).into_iter() {
            self.spi.write(&led_value.as_array())?;
        }
        // Send a null message to finish the message
        self.spi.write(&NULL_MESSAGE)?;
        Ok(self)
    }
}

impl Drop for LedInterface {
    fn drop(&mut self) {
        let _ = self.clear().flush();
    }
}
