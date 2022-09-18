use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub duration: i32,
    pub record_side: String,
    pub digital: bool,
    pub urls: String,
}
