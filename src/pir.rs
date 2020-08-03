use crate::Result;
use crossbeam_channel::{bounded, Receiver};
use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use std::{thread, thread::JoinHandle};

pub struct PassiveInfraRedSensor {
    handle: JoinHandle<Result<()>>,
    receiver: Receiver<bool>,
}

impl PassiveInfraRedSensor {
    pub fn new(pin: u8) -> Result<Self> {
        let (sender, receiver) = bounded::<bool>(1);
        let handle = thread::spawn(move || {
            let mut input_pin = Gpio::new()?.get(pin)?.into_input();
            input_pin.set_interrupt(Trigger::Both)?;
            loop {
                let level = input_pin.poll_interrupt(false, None)?;
                match level {
                    Some(Level::High) => {
                        let _ = sender.send(true);
                    }
                    Some(Level::Low) => {
                        let _ = sender.send(false);
                    }
                    _ => {}
                }
            }
            Ok(())
        });
        Ok(Self { handle, receiver })
    }

    pub fn get_receiver(self) -> Receiver<bool> {
        self.receiver.clone()
    }
}
