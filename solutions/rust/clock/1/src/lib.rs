use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut offset = (60 * hours + minutes) % (24 * 60);
        if offset < 0 {
            offset += 24 * 60;
        }
        Clock {
            hours: offset / 60,
            minutes: offset % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let add_hours = minutes / 60;
        let add_minutes = minutes % 60;
        Clock::new(self.hours + add_hours, self.minutes + add_minutes)
    }
}
