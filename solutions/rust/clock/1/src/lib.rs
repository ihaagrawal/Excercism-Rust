use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    hours: i32,
    minutes: i32,
    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total = 60 * hours + minutes;
        Self::from_minutes(total)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total = self.hours * 60 + self.minutes + minutes;
        Self::from_minutes(total)
    }

    pub fn from_minutes(total: i32) -> Self{
        let minutes_in_day = 24 * 60;
        let mut newtotal = total % minutes_in_day;
        if newtotal < 0{
            newtotal += minutes_in_day;
        }

        let hours = newtotal / 60;
        let minutes = newtotal % 60;

        Clock {hours, minutes}
    }
}
