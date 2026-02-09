use std::{fmt::Display, str::FromStr};

use serde::{Serialize, Deserialize};
use chrono::{Datelike, Local, Timelike, Weekday};

use crate::Location;
// use chrono // figure out usage

// 1-indexed to match unix `date +%u` output
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Day {
    #[default]
    Async,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    pub fn tomorrow(self) -> Day {
        match self {
            Day::Async => Day::Async,
            Day::Monday => Day::Tuesday,
            Day::Tuesday => Day::Wednesday,
            Day::Wednesday => Day::Thursday,
            Day::Thursday => Day::Friday,
            Day::Friday => Day::Saturday,
            Day::Saturday => Day::Sunday,
            Day::Sunday => Day::Monday,
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
                Day::Async     => "Asynchronous",
                Day::Monday    => "Monday",
                Day::Tuesday   => "Tuesday",
                Day::Wednesday => "Wednesday",
                Day::Thursday  => "Thursday",
                Day::Friday    => "Friday",
                Day::Saturday  => "Saturday",
                Day::Sunday    => "Sunday",
            }
        )
    }
}

impl FromStr for Day {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let d = s.to_ascii_lowercase();
        let d = d.trim();

        match d {
            "sa" | "sat" | "saturday"       => Ok(Day::Saturday), // means that "S" will result in "Saturday"
            "su" | "sun" | "sunday"         => Ok(Day::Sunday),
            _ if "monday".starts_with(d)    => Ok(Day::Monday),
            _ if "tuesday".starts_with(d)   => Ok(Day::Tuesday),
            _ if "wednesday".starts_with(d) => Ok(Day::Wednesday),
            _ if "thursday".starts_with(d)  => Ok(Day::Thursday),
            _ if "friday".starts_with(d)    => Ok(Day::Friday),
            _ if d.trim().len() == 0        => Ok(Day::Async),
            _ => Err(format!("Unable to parse date from input: \"{s}\""))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
pub struct Time {
    pub day: Day,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn now() -> Self {
        let now = Local::now();

        let day = match now.weekday() {
            Weekday::Mon => Day::Monday,
            Weekday::Tue => Day::Tuesday,
            Weekday::Wed => Day::Wednesday,
            Weekday::Thu => Day::Thursday,
            Weekday::Fri => Day::Friday,
            Weekday::Sat => Day::Saturday,
            Weekday::Sun => Day::Sunday,
        };

        Time {
            day,
            hour: now.hour() as u8,
            minute: now.minute() as u8,
            second: now.second() as u8,
        }
    }

    fn to_absolute_seconds(&self) -> u32 {
        // guard in caller for Async, otherwise it will be treated as Monday
        let day = (self.day as u32).saturating_sub(1); // - 1 to account for 1-indexing in enum
        let hour = self.hour as u32;
        let minute = self.minute as u32;
        let second = self.second as u32;

        (60 * 60 * 24 * day) + (60 * 60 * hour) + (60 * minute) + second
    }

    fn is_between(&self, start: &Time, end: &Time) -> bool {
        if self.day == Day::Async || start.day == Day::Async || end.day == Day::Async {
            return true;
        }

        let curr_secs = self.to_absolute_seconds();
        let start_secs = start.to_absolute_seconds();
        let end_secs = end.to_absolute_seconds();

        if start_secs <= end_secs {
            // if the start and in fall in the same week cycle
            curr_secs >= start_secs && curr_secs <= end_secs
        } else {
            // if the range wraps between week cycles
            curr_secs >= start_secs || curr_secs <= end_secs
        }
    }
    
    pub fn hms_string(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
    }
    
    pub fn get_hms(&self) -> (u8, u8, u8) {
        (self.hour, self.minute, self.second)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} on {}", self.hms_string(), self.day))
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Times {
    #[default]
    Async,
    At(Vec<ClassTime>)
}

impl From<Vec<ClassTime>> for Times {
    fn from(value: Vec<ClassTime>) -> Self {
        if value.is_empty() {
            Self::Async
        } else {
            Self::At(value)
        }
    }
}

impl Times {
    pub fn len(&self) -> usize {
        match &self {
            Self::Async => 0,
            Self::At(times) => times.len()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct ClassTime {
    pub start: Time,
    pub end: Time,
    pub location: Location,
}

impl From<(Time, Time, Location)> for ClassTime {
    fn from(value: (Time, Time, Location)) -> Self {
        Self {
            start: value.0,
            end: value.1,
            location: value.2
        }
    }
}

impl ClassTime {
    pub fn includes(&self, time: Time) -> bool {
        // let (start_hours, start_mins, start_secs) = self.start.get_hms();
        // let start = NaiveTime::from_hms_opt(start_hours as u32, start_mins as u32, start_secs as u32).unwrap();

        // let (end_hours, end_mins, end_secs) = self.start.get_hms();
        // let end = NaiveTime::from_hms_opt(end_hours as u32, end_mins as u32, end_secs as u32).unwrap();
        
        time.is_between(&self.start, &self.end)
    }
}