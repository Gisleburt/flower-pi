mod clock;
mod error;
mod led;
mod pollen;
mod signal;

use crate::clock::Clock;
use crate::error::{ErrorHandler, Result};
use crate::led::{LedClock, LedInterface};
use crate::pollen::{get_pollen_count, PollenCount};
use crate::signal::Signal;
use crossbeam_channel::{bounded, select, tick, Sender};
use std::time::Duration;
use std::{env, thread};

fn main() {
    App::new().unwrap().run();
}

struct App {
    interface: LedInterface,
    led_clock: LedClock,
    error_handler: ErrorHandler,
}

impl App {
    pub fn new() -> Result<App> {
        let error_handler = ErrorHandler::new(&env::var("IFTTT_KEY").unwrap());
        let clock = Clock::new();
        let led_clock = LedClock::new(24, 12, clock);
        match LedInterface::new(24) {
            Err(error) => {
                error_handler.handle_error(&error);
                panic!("{:?}", error);
            }
            Ok(interface) => Ok(App {
                interface,
                led_clock,
                error_handler,
            }),
        }
    }

    fn update_pollen_count(sender: Sender<Option<PollenCount>>) {
        // Warning: This process is immediately orphaned
        thread::spawn(move || {
            let _ = sender.send(get_pollen_count().ok());
        });
    }

    pub fn run(&mut self) {
        let mut error_count = 0;
        while error_count < 5 {
            match self.enter_render_loop() {
                Ok(_) => break,
                Err(e) => {
                    self.error_handler.handle_error(&e);
                    error_count += 1;
                }
            }
        }
    }

    pub fn enter_render_loop(&mut self) -> Result<()> {
        let (pollen_sender, pollen_receiver) = bounded::<Option<PollenCount>>(1);
        let sig_receiver = Signal::get_exit_receiver();
        let render = tick(Duration::from_millis(100));
        let update_pollen_count = tick(Duration::from_secs(60 * 60));

        App::update_pollen_count(pollen_sender.clone()); // One off run
        loop {
            select! {
                recv(sig_receiver) -> _ => {
                    return Ok(());
                }
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
