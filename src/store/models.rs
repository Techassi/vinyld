use std::collections::HashMap;

use sqlx::types::time::PrimitiveDateTime;

use crate::types::{Artist, BuyCondition, Condition, MediaType};

pub struct MediaJoin {
    pub media_id: String,
    pub media_title: String,
    pub media_media_type: MediaType,
    pub media_catalogue: String,
    pub media_release_date: PrimitiveDateTime,
    pub media_purchase_date: PrimitiveDateTime,
    pub media_media_condition: Condition,
    pub media_sleeve_condition: Condition,
    pub media_bought: BuyCondition,
    pub media_created_at: PrimitiveDateTime,
    pub media_modified_at: PrimitiveDateTime,
    pub media_notes: String,
    // Label
    pub label_id: String,
    pub label_name: String,
    pub label_label_code: String,
    pub label_urls: String,
}

pub struct ArtistsJoin {
    pub artist_id: String,
    pub artist_name: String,
    pub artist_urls: String,
    pub media_id: String,
}

pub struct TracksJoin {
    pub track_id: String,
    pub track_title: String,
    pub track_duration: i32,
    pub track_record_side: String,
    pub track_digital: bool,
    pub track_urls: String,
    pub media_id: String,
}
