use std::fmt::Display;

use super::time::Times;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Class {
    name: String,
    times: Times,
    professor: String,
    office_hours: Times 
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match &self.times {
                Times::Async => format!("\t{} ({}), asynchronous.", self.name, self.professor),
                Times::At(times) => times.iter().fold(String::new(), |init, time|
                    format!("{init}\t{name} ({prof}) on {day}, from {start} to {end}, at {camp}, {bldg} (Room {room})\n",
                        name  = self.name,
                        prof  = self.professor,
                        camp  = time.location.campus,
                        bldg  = time.location.building,
                        room  = time.location.room,
                        start = time.start.hms_string(),
                        end   = time.end.hms_string(),
                        day   = time.start.day
                    )
                )
            }.as_str()
        )
    }
}

impl Class {
    pub fn from_names(class: String, professor: String) -> Class {
        Class {
            name: class,
            professor,
            ..Default::default()
        }        
    }

    pub fn new(name: String, professor: String, times: Times, office_hours: Times) -> Class {
         Class {
            name,
            times,
            professor,
            office_hours,
         }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_professor(&self) -> &String {
        &self.professor
    }

    pub fn get_times(&self) -> &Times {
        &self.times
    }
}