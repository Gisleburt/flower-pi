mod clock;
mod led;
mod pollen;

use crate::led::{LedClock, LedInterface};
use crate::pollen::{get_pollen_count, PollenCount};
use crate::clock::Clock;
use core::fmt;
use std::error::Error;
use std::thread;
use std::time::Duration;
use crossbeam_channel::{tick, select, bounded, Sender};

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
    App::new().unwrap().run().unwrap();
}

struct App {
    interface: LedInterface,
    led_clock: LedClock,
}

impl App {
    pub fn new() -> Result<App> {
        let clock = Clock::new();
        let led_clock = LedClock::new(24, 12, clock);
        let interface = LedInterface::new(24)?;
        Ok(App {
            interface,
            led_clock,
        })
    }

    fn update_pollen_count(sender: Sender<Option<PollenCount>>) {
        // Warning: This process is immediately orphaned
        thread::spawn(move || {
            let _ = sender.send(get_pollen_count().ok());
        });
    }

    pub fn run(&mut self) -> Result<()> {
        let render = tick(Duration::from_millis(100));
        let (pollen_sender, pollen_receiver) = bounded::<Option<PollenCount>>(100);
        App::update_pollen_count(pollen_sender.clone());
        let update_pollen_count = tick(Duration::from_secs(60 * 60));
        loop {
            select! {
                recv(render) -> _ => {
                    self.led_clock.update()?;
                    self.interface.write(&self.led_clock)?.flush()?;
                }
                recv(pollen_receiver) -> pollen_result => {
                    match pollen_result {
                        Err(e) => panic!(e), // ToDo: Handle this
                        Ok(pollen_count) => self.led_clock.set_background(pollen_count.into()),
                    };
                }
                recv(update_pollen_count) -> _ => {
                    App::update_pollen_count(pollen_sender.clone());
                }
            }
        }
    }
}
