use std::fmt::Display;
use serde::{Serialize, Deserialize};

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
    Unset
}

impl Day {
    pub fn tomorrow(&self) -> Day {
        match *self {
            Day::Async => Day::Async,
            Day::Monday => Day::Tuesday,
            Day::Tuesday => Day::Wednesday,
            Day::Wednesday => Day::Thursday,
            Day::Thursday => Day::Friday,
            Day::Friday => Day::Saturday,
            Day::Saturday => Day::Sunday,
            Day::Sunday => Day::Monday,
            Day::Unset => Day::Unset,
        }
    }
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        let lc_val = value.to_ascii_lowercase();
        match &lc_val {
            d if "monday".starts_with(d)    => Day::Monday,
            d if "tuesday".starts_with(d)   => Day::Tuesday,
            d if "wednesday".starts_with(d) => Day::Wednesday,
            d if "thursday".starts_with(d)  => Day::Thursday,
            d if "friday".starts_with(d)    => Day::Friday,
            d if "saturday".starts_with(d)  => Day::Saturday, // means that "S" will result in "Saturday"
            d if "sunday".starts_with(d)    => Day::Sunday,
            d if d.trim().len() == 0        => Day::Async,
            _ => Day::Unset
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
                Day::Unset     => "Undefined",
            }
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct Time {
    pub day: Day,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ClassTime {
    pub start: Time,
    pub end: Time,
}

impl From<(Time, Time)> for ClassTime {
    fn from(value: (Time, Time)) -> Self {
        Self {
            start: value.0,
            end: value.1
        }
    }
}