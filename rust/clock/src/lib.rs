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
        let mut hours = if minutes / 60 >= 1 {
            hours + minutes / 60
        } else {
            hours
        };

        let minutes = if { minutes } < 0 {
            let mut tmp_minutes = minutes;

            let upper_bound = if minutes == -60 {
                minutes / 60 * -1
            } else {
                minutes / 60 * -1 + 1
            };

            for _ in 0..upper_bound {
                tmp_minutes += 60;
                hours -= 1;
            }
            tmp_minutes
        } else {
            minutes
        };

        let hours = if hours < 0 {
            let mut tmp_hours = hours;
            for _ in 0..hours / 24 * -1 + 1 {
                tmp_hours += 24;
            }
            tmp_hours
        } else {
            hours
        };

        Clock {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_minutes = self.minutes + minutes;
        let mut hours = if new_minutes / 60 >= 1 {
            self.hours + new_minutes / 60
        } else {
            self.hours
        };

        let minutes = if { new_minutes } < 0 {
            let mut tmp_minutes = new_minutes;

            let upper_bound = if new_minutes == -60 {
                new_minutes / 60 * -1
            } else {
                new_minutes / 60 * -1 + 1
            };

            for _ in 0..upper_bound {
                tmp_minutes += 60;
                hours -= 1;
            }
            tmp_minutes
        } else {
            new_minutes % 60
        };

        let hours = if hours < 0 {
            let mut tmp_hours = hours;
            for _ in 0..hours / 24 * -1 + 1 {
                tmp_hours += 24;
            }
            tmp_hours
        } else {
            hours
        };

        Clock {
            hours: hours % 24,
            minutes: minutes % 60,
        }
    }
}
