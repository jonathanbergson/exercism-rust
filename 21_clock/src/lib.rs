use std::fmt;

const MINUTES_OF_DAY: i32 = 1440; // 60min * 24h
const MINUTES_OF_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let input = (hours * MINUTES_OF_HOUR) + minutes;
        Clock::build(input)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let input = self.minutes + minutes;
        Clock::build(input)
    }

    fn build(minutes: i32) -> Self {
        Clock {
            minutes: minutes.rem_euclid(MINUTES_OF_DAY),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes.div_euclid(MINUTES_OF_HOUR);
        let minutes = self.minutes.rem_euclid(MINUTES_OF_HOUR);
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
