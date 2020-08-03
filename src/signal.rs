use core::fmt;
use crossbeam_channel::{bounded, Receiver};
use signal_hook::iterator::Signals;
use signal_hook::{SIGALRM, SIGHUP, SIGINT, SIGPIPE, SIGPROF, SIGTERM, SIGUSR1, SIGUSR2};
use std::thread;

pub struct Signal(i32);

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            SIGALRM => write!(f, "SIGALRM"),
            SIGHUP => write!(f, "SIGHUP"),
            SIGINT => write!(f, "SIGINT"),
            SIGPIPE => write!(f, "SIGPIPE"),
            SIGPROF => write!(f, "SIGPROF"),
            SIGTERM => write!(f, "SIGTERM"),
            SIGUSR1 => write!(f, "SIGUSR1"),
            SIGUSR2 => write!(f, "SIGUSR2"),
            x => write!(f, "UNKNOWN: {}", x),
        }
    }
}

impl Signal {
    pub fn get_exit_receiver() -> Receiver<i32> {
        // Warning: This process is immediately orphaned
        let (signal_sender, signal_receiver) = bounded::<i32>(10);
        thread::spawn(move || {
            let signals = Signals::new(&[
                SIGALRM, SIGHUP, SIGINT, SIGPIPE, SIGPROF, SIGTERM, SIGUSR1, SIGUSR2,
            ])
            .unwrap();
            for signal in signals.forever() {
                match signal {
                    SIGALRM | SIGHUP | SIGINT | SIGPIPE | SIGPROF | SIGTERM | SIGUSR1 | SIGUSR2 => {
                        println!("Received {}", Signal(signal));
                        break;
                    }
                    _ => println!("unknown signal received"),
                }
                let _ = signal_sender.send(signal); // We're quitting now, not a lot else to do
            }
        });
        signal_receiver
    }
}
