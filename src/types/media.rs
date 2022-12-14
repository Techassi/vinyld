use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use sqlx::{types::time::PrimitiveDateTime, Type};

use crate::{
    store::models::MediaJoin,
    types::{Artist, BuyCondition, Condition, Label, Track},
};

#[derive(Serialize)]
pub struct Media {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,
    pub artists: Vec<Artist>,
    pub label: Label,
    pub catalogue: String,
    pub tracks: Vec<Track>,
    pub release_date: String,
    pub purchase_date: String,
    pub media_condition: Condition,
    pub sleeve_condition: Condition,
    pub bought: BuyCondition,
    pub created_at: String,
    pub modified_at: String,
    pub notes: String,
}

impl From<MediaJoin> for Media {
    fn from(mj: MediaJoin) -> Self {
        Self {
            id: mj.media_id,
            title: mj.media_title,
            media_type: mj.media_media_type,
            artists: Vec::new(),
            label: Label {
                id: mj.label_id,
                name: mj.label_name,
                label_code: mj.label_label_code,
                urls: mj.label_urls,
            },
            catalogue: mj.media_catalogue,
            tracks: Vec::new(),
            release_date: mj.media_release_date.to_string(),
            purchase_date: mj.media_purchase_date.to_string(),
            media_condition: mj.media_media_condition,
            sleeve_condition: mj.media_sleeve_condition,
            bought: mj.media_bought,
            created_at: mj.media_created_at.to_string(),
            modified_at: mj.media_modified_at.to_string(),
            notes: mj.media_notes,
        }
    }
}
impl From<RawMedia> for Media {
    fn from(rm: RawMedia) -> Self {
        Self {
            id: rm.id,
            title: rm.title,
            media_type: rm.media_type,
            artists: Vec::new(),
            label: Label::default(),
            catalogue: rm.catalogue,
            tracks: Vec::new(),
            release_date: rm.release_date.to_string(),
            purchase_date: rm.purchase_date.to_string(),
            media_condition: rm.media_condition,
            sleeve_condition: rm.sleeve_condition,
            bought: rm.bought,
            created_at: rm.created_at.to_string(),
            modified_at: rm.modified_at.to_string(),
            notes: rm.notes,
        }
    }
}

#[derive(Type)]
pub struct RawMedia {
    pub id: String,
    pub title: String,
    pub media_type: MediaType,
    pub catalogue: String,
    pub release_date: PrimitiveDateTime,
    pub purchase_date: PrimitiveDateTime,
    pub media_condition: Condition,
    pub sleeve_condition: Condition,
    pub bought: BuyCondition,
    pub created_at: PrimitiveDateTime,
    pub modified_at: PrimitiveDateTime,
    pub notes: String,
}

#[derive(Serialize, Type, Clone)]
pub enum MediaType {
    Vinyl,
    Tape,
    Cd,
}

impl From<String> for MediaType {
    fn from(input: String) -> Self {
        match input.to_lowercase().as_str() {
            "vinyl" => Self::Vinyl,
            "tape" => Self::Tape,
            "cd" => Self::Cd,
            _ => Self::Vinyl,
        }
    }
}

#[derive(Serialize)]
pub struct GetMediaEntriesResponse {
    status: String,
    entries: Vec<Media>,
}

impl GetMediaEntriesResponse {
    pub fn success(entries: Vec<Media>) -> Self {
        Self {
            status: String::from("success"),
            entries,
        }
    }
}

impl IntoResponse for GetMediaEntriesResponse {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(self));
        body.into_response()
    }
}

#[derive(Serialize)]
pub struct GetMediaEntryResponse {
    status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    entry: Option<Media>,
}

impl GetMediaEntryResponse {
    pub fn success(entry: Media) -> Self {
        Self {
            status: String::from("success"),
            entry: Some(entry),
            error: None,
        }
    }

    pub fn error<M: Into<String>>(message: M) -> Self {
        Self {
            status: String::from("error"),
            error: Some(message.into()),
            entry: None,
        }
    }
}

impl IntoResponse for GetMediaEntryResponse {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(self));
        body.into_response()
    }
}
