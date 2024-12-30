use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{:02}:{:02}", self.hours, self.minutes).fmt(f)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = Clock::adjust_hours_for_minutes(hours, minutes);
        let (minutes, hours) = Clock::calculate_minutes(minutes, hours);
        let hours = Clock::calculate_hours(hours);

        Clock {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let hours = Clock::adjust_hours_for_minutes(self.hours, minutes);
        let (minutes, hours) = Clock::calculate_minutes(minutes, hours);
        let hours = Clock::calculate_hours(hours);

        Clock {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    fn calculate_hours(hours: i32) -> i32 {
        match hours < 0 {
            true => {
                let mut tmp_hours = hours;
                for _ in 0..hours / 24 * -1 + 1 {
                    tmp_hours += 24;
                }
                tmp_hours
            }
            false => hours,
        }
    }

    fn calculate_minutes(minutes: i32, mut hours: i32) -> (i32, i32) {
        match minutes {
            m if m < 0 => {
                let adjustment = (-m + 59) / 60;
                let adjusted_minutes = m + adjustment * 60;
                hours -= adjustment;
                (adjusted_minutes, hours)
            }
            m => (m % 60, hours),
        }
    }

    fn adjust_hours_for_minutes(hours: i32, minutes: i32) -> i32 {
        match minutes / 60 >= 1 {
            true => hours + minutes / 60,
            false => hours,
        }
    }
}
