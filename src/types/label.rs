use serde::Serialize;

#[derive(Serialize)]
pub struct Label {
    id: String,
    name: String,
    label_code: String,
    urls: String,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            label_code: Default::default(),
            urls: Default::default(),
        }
    }
}
