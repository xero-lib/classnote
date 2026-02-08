use std::{fmt::Display, str::FromStr};
use serde::{Serialize, Deserialize};

use crate::Location;
// use chrono // figure out usage

// 1-indexed to match unix `date +%u` output
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Copy)]
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

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} on {}", self.get_hms(), self.day))
    }
}

impl Time {
    pub fn get_hms(&self) -> String {
        format!("{:0>2}:{:0>2}:{:0>2}", self.hour, self.minute, self.second)
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