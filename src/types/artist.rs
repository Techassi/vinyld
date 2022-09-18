use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub urls: String,
}
