use chrono::{Local, Timelike};

pub struct Clock;

impl Clock {
    pub fn new() -> Clock {
        Clock
    }

    pub fn get_seconds(&self) -> usize {
        Local::now().second() as usize
    }

    pub fn get_minutes(&self) -> usize {
        Local::now().minute() as usize
    }

    pub fn get_hours(&self) -> usize {
        Local::now().hour() as usize
    }
}
