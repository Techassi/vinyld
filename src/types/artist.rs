use serde::Serialize;

#[derive(Serialize)]
pub struct Artist {
    id: String,
    name: String,
    urls: String,
}
