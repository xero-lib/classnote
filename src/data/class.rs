use crate::data::{
    time::{ClassTime, Time},
    location::Location
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Class {
    pub name: String,
    pub times: Vec<ClassTime>,
    pub professor: String,
    pub location: Location,
    pub office_hours: Vec<(Location, Time)>
}