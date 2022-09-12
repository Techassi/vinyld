use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Track {
    title: String,
    label: String,
    length: usize,
}
