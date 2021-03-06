use crate::error::FlowerError;
use crate::led::LedMessage;
use crate::pollen::PollenCount;
use crate::Result;

#[derive(Clone, Copy, Debug)]
pub struct LedValue {
    brightness: u8,
    blue: u8,
    green: u8,
    red: u8,
}

impl LedValue {
    pub fn new(brightness: u8, red: u8, green: u8, blue: u8) -> Result<LedValue> {
        if brightness > 31u8 {
            Err(FlowerError::SimpleError("brightness can not be higher than 31".to_string()).into())
        } else {
            Ok(LedValue {
                brightness,
                red,
                green,
                blue,
            })
        }
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

pub const LED_LOW_RED: LedValue = LedValue {
    brightness: 1,
    red: 255,
    green: 0,
    blue: 0,
};
pub const LED_LOW_YELLOW: LedValue = LedValue {
    brightness: 1,
    red: 255,
    green: 150,
    blue: 0,
};
pub const LED_LOW_GREEN: LedValue = LedValue {
    brightness: 1,
    red: 0,
    green: 255,
    blue: 0,
};

pub const LED_LOW_BLUE: LedValue = LedValue {
    brightness: 1,
    red: 0,
    green: 0,
    blue: 255,
};
pub const LED_LOW_PURPLE: LedValue = LedValue {
    brightness: 1,
    red: 255,
    green: 0,
    blue: 255,
};
pub const LED_LOW_AQUA: LedValue = LedValue {
    brightness: 1,
    red: 0,
    green: 255,
    blue: 255,
};

impl From<Option<PollenCount>> for LedValue {
    fn from(count: Option<PollenCount>) -> Self {
        match count {
            Some(PollenCount::High) => LED_LOW_RED,
            Some(PollenCount::Medium) => LED_LOW_YELLOW,
            Some(PollenCount::Low) => LED_LOW_GREEN,
            None => LedValue::default(),
        }
    }
}
