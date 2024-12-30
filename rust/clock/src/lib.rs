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
            hours: hours.rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let hours = Clock::adjust_hours_for_minutes(self.hours, minutes);
        let (minutes, hours) = Clock::calculate_minutes(minutes, hours);
        let hours = Clock::calculate_hours(hours);

        Clock {
            hours: hours.rem_euclid(24),
            minutes: minutes.rem_euclid(60),
        }
    }

    fn calculate_hours(hours: i32) -> i32 {
        match hours {
            h if h < 0 => h.rem_euclid(24),
            h => h,
        }
    }

    fn calculate_minutes(minutes: i32, mut hours: i32) -> (i32, i32) {
        match minutes {
            // use pattern guard
            m if m < 0 => {
                let adjustment = (-m + 59) / 60;
                let adjusted_minutes = m + adjustment * 60;
                hours -= adjustment;
                (adjusted_minutes.rem_euclid(60), hours)
            }
            m => (m.rem_euclid(60), hours),
        }
    }

    fn adjust_hours_for_minutes(hours: i32, minutes: i32) -> i32 {
        match minutes.div_euclid(60) {
            // use pattern guard
            m if m >= 1 => hours + m,
            _ => hours,
        }
    }
}
