use serde::Serialize;

#[derive(Serialize)]
pub struct Label {
    pub id: String,
    pub name: String,
    pub label_code: String,
    pub urls: String,
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
