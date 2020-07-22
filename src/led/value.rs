use crate::led::LedMessage;
use crate::{DumbError, Result};

#[derive(Clone, Debug)]
pub struct LedValue {
    brightness: u8,
    blue: u8,
    green: u8,
    red: u8,
}

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
