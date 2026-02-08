use super::time::ClassTime;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Class {
    name: String,
    times: Vec<ClassTime>,
    professor: String,
    office_hours: Vec<ClassTime>
}

impl Class {
    pub fn from_names(class: String, professor: String) -> Class {
        Class {
            name: class,
            professor,
            ..Default::default()
        }        
    }

    pub fn new(name: String, professor: String, times: Vec<ClassTime>, office_hours: Vec<ClassTime>) -> Class {
         Class {
            name,
            times,
            professor,
            office_hours,
         }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }
}