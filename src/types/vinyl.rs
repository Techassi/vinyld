use std::time;

use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::types::{BuyCondition, Condition, Track};

#[derive(Serialize)]
pub struct Vinyl {
    pub id: String,
    pub title: String,
    pub artists: Vec<String>,
    pub release_year: usize,
    pub tracks: Vec<Track>,
    // pub bought_at: time::Instant,
    pub bought_in_condition: BuyCondition,
    pub media_condition: Condition,
    pub sleeve_condition: Condition,
    pub digital_files_included: bool,
}

#[derive(Deserialize)]
pub struct CreateVinylRequest {
    pub title: String,
    pub artists: Vec<String>,
    pub release_year: usize,
    pub tracks: Vec<Track>,
    // pub bought_at: time::Instant,
    pub bought_in_condition: BuyCondition,
    pub media_condition: Condition,
    pub sleeve_condition: Condition,
    pub digital_files_included: bool,
}

#[derive(Serialize)]
pub struct CreateVinylResponse {
    pub message: String,
    pub status: String,
}

impl CreateVinylResponse {
    pub fn success<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
            status: String::from("success"),
        }
    }

    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
            status: String::from("error"),
        }
    }
}

impl IntoResponse for CreateVinylResponse {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(self));
        body.into_response()
    }
}
