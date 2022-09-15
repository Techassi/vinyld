use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Track {
    id: String,
    title: String,
    duration: usize,
    record_side: String,
    digital: bool,
    urls: String,
}
