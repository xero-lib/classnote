use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct Location {
    pub campus: String,
    pub building: String,
    pub room: String,
}
