pub trait Clock {
    fn get_seconds(&self) -> usize;
    fn get_minutes(&self) -> usize;
    fn get_hours(&self) -> usize;
}

#[derive(Default)]
pub struct FakeClock {
    pub seconds: usize,
    pub minutes: usize,
    pub hours: usize,
}

impl FakeClock {
    pub fn new() -> FakeClock {
        FakeClock::default()
    }

    pub fn advance_one_second(&mut self) {
        self.seconds = self.seconds + 1;
        if self.seconds >= 60 {
            self.seconds = 0;
            self.advance_one_minute();
        }
    }

    pub fn advance_one_minute(&mut self) {
        self.minutes = self.minutes + 1;
        if self.minutes >= 60 {
            self.minutes = 0;
            self.advance_one_hour();
        }
    }

    pub fn advance_one_hour(&mut self) {
        self.hours = self.hours + 1;
        if self.hours >= 24 {
            self.hours = 0;
        }
    }
}

impl Clock for FakeClock {
    fn get_seconds(&self) -> usize {
        self.seconds
    }

    fn get_minutes(&self) -> usize {
        self.minutes
    }

    fn get_hours(&self) -> usize {
        self.hours
    }
}
